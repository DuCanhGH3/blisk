use super::{
    auth::{OptionalUserClaims, UserClaims},
    posts::Post,
};
use crate::{
    app::AppState,
    settings::SETTINGS,
    utils::{
        constants::SLUG_REGEX,
        errors::AppError,
        response::{created, response},
        structs::{AppForm, AppImage, AppJson, AppMultipart, AppQuery},
        uploads::upload_file,
    },
};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::NaiveDate;
use tracing::instrument;
use validator::{Validate, ValidationError};

#[derive(serde::Serialize)]
pub struct BooksMetadata {
    authors: sqlx::types::Json<Vec<BookAuthor>>,
    categories: sqlx::types::Json<Vec<BookCategory>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BookReactionMetadata {
    total: i64,
    like: i64,
    dislike: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum BooksError {
    #[error("duplicate slug {0}")]
    SlugAlreadyExists(String),
    #[error("language {0} does not exist")]
    LanguageInvalid(String),
    #[error("book {0} cannot be found")]
    BookNotFound(String),
    #[error("user is already tracking book {0}")]
    AlreadyTracking(String),
    #[error("this error is not expected")]
    Unexpected,
}

#[derive(TryFromMultipart, Validate)]
pub struct CreatePayload {
    #[validate(length(min = 1, message = "Title must not be empty!"))]
    title: String,
    #[validate(length(min = 1), regex(path = *SLUG_REGEX, message = "Slug is not valid! It must only contain ASCII characters, numbers, and/or hyphens."))]
    slug: String,
    #[validate(range(min = 0, message = "Number of pages must be a number!"))]
    pages: i32,
    #[validate(length(min = 1, message = "Language is not valid!"))]
    language: String,
    #[validate(length(min = 1, message = "Synopsis must not be empty!"))]
    summary: String,
    #[validate(length(min = 1, message = "Book must has at least one author!"))]
    authors: Vec<i64>,
    #[validate(length(min = 1, message = "Book must has at least one category!"))]
    categories: Vec<i64>,
    #[form_data(limit = "2000000")]
    cover_image: FieldData<Bytes>,
    #[form_data(limit = "2000000")]
    spine_image: FieldData<Bytes>,
}

#[instrument(name = "Creating a new book...", skip(pool, s3, claims, cover_image, spine_image), fields(uid = %claims.sub))]
pub async fn create(
    State(AppState { pool, s3, .. }): State<AppState>,
    claims: UserClaims,
    AppMultipart(CreatePayload {
        title,
        slug,
        pages,
        language,
        summary,
        authors,
        categories,
        cover_image,
        spine_image,
    }): AppMultipart<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let cover_id = upload_file(&mut transaction, &s3, claims.sub, None, cover_image).await?;
    let spine_id = upload_file(&mut transaction, &s3, claims.sub, None, spine_image).await?;
    let bid: i64 = sqlx::query_scalar!(
        "INSERT INTO books (is_approved, title, name, pages, language, summary, cover_id, spine_id)
        VALUES (FALSE, $1, $2, $3, $4, $5, $6, $7)
        RETURNING id",
        &title,
        &slug,
        &pages,
        &language,
        &summary,
        &cover_id,
        &spine_id,
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(ref db_err) => match db_err.constraint() {
            Some("books_name_key") => AppError::from(BooksError::SlugAlreadyExists(slug.clone())),
            Some("books_language_fkey") => AppError::from(BooksError::LanguageInvalid(language)),
            _ => AppError::from(err),
        },
        err => AppError::from(err),
    })?;
    sqlx::query!(
        "INSERT INTO book_to_author (book_id, author_id) SELECT $1, * FROM UNNEST($2::BIGINT[])",
        &bid,
        &authors[..]
    )
    .execute(&mut *transaction)
    .await?;
    sqlx::query!(
        "INSERT INTO book_to_category (book_id, category_id) SELECT $1, * FROM UNNEST($2::BIGINT[])",
        &bid,
        &categories[..]
    )
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(created(format!("{}/books/{}", SETTINGS.frontend.url, slug)))
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BookAuthor {
    id: i64,
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BookCategory {
    pub id: i64,
    pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Book {
    pub title: String,
    pub name: String,
    pub summary: String,
    pub language: String,
    pub cover_image: sqlx::types::Json<AppImage>,
    pub spine_image: sqlx::types::Json<AppImage>,
    pub authors: sqlx::types::Json<Vec<BookAuthor>>,
    pub categories: sqlx::types::Json<Vec<BookCategory>>,
    pub reactions: Option<sqlx::types::Json<BookReactionMetadata>>,
    pub reviews: Option<sqlx::types::Json<Vec<Post>>>,
}

#[derive(serde::Deserialize, Validate)]
pub struct ReadQuery {
    /// Whether reviews should be included with books.
    include_reviews: Option<bool>,
    #[validate(length(min = 1, message = "Query is not valid!"))]
    q: Option<String>,
    #[validate(length(min = 1, message = "No categories are specified!"))]
    categories: Option<Vec<i64>>,
    #[validate(length(min = 1, message = "No authors are specified!"))]
    authors: Option<Vec<i64>>,
}

#[instrument(name = "Reading books...", skip(pool, claims))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    AppQuery(ReadQuery {
        include_reviews,
        q,
        categories,
        authors,
    }): AppQuery<ReadQuery>,
) -> Result<Response, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let include_reviews = include_reviews.unwrap_or(true);
    let books_list = sqlx::query_as!(
        Book,
        r#"
        SELECT
            b.title AS "title!",
            b.name AS "name!",
            b.summary AS "summary!",
            b.lang AS "language!",
            b.cover_image AS "cover_image!: _",
            b.spine_image AS "spine_image!: _",
            b.authors AS "authors!: _",
            b.categories AS "categories!: _",
            b.reactions AS "reactions?: _",
            CASE $5::BOOLEAN
                WHEN TRUE THEN coalesce(jsonb_agg(rv) FILTER (WHERE rv.id IS NOT NULL), '[]'::JSONB)
                ELSE NULL
            END AS "reviews?: _"
        FROM books_view b
        LEFT JOIN LATERAL (
            SELECT *
            FROM fetch_posts(request_uid => $1) rv
            WHERE rv.book_id = b.id
            ORDER BY rv.id DESC
            LIMIT 5
            OFFSET 0
        ) rv ON TRUE
        LEFT JOIN LATERAL (
            SELECT books_boost_rating(brt) AS rating
            FROM book_reactions_tally brt
            WHERE brt.book_id = b.id
        ) bbr ON TRUE
        JOIN LATERAL (
            SELECT CASE
                WHEN $2::TEXT IS NOT NULL THEN websearch_to_tsquery(coalesce($2, ''))
                ELSE NULL
            END AS query
        ) query ON TRUE
        JOIN LATERAL (
            SELECT CASE
                WHEN query.query IS NULL THEN 0
                ELSE ts_rank(b.text_search, query.query)
            END + CASE
                WHEN bbr IS NULL THEN 0
                ELSE bbr.rating
            END AS rank
        ) rank ON TRUE
        WHERE CASE
            WHEN query.query IS NULL THEN TRUE
            ELSE b.text_search @@ query.query
        END AND CASE
            WHEN $3::BIGINT[] IS NULL THEN TRUE
            ELSE b.categories_raw @> $3
        END AND CASE
            WHEN $4::BIGINT[] IS NULL THEN TRUE
            ELSE b.authors_raw @> $4
        END
        GROUP BY b.title, b.name, b.summary, b.lang, b.cover_image,
        b.spine_image, b.authors, b.categories, b.reactions, rank.rank
        ORDER BY rank.rank DESC"#,
        &uid as &_,
        &q as &_,
        &categories as &_,
        &authors as &_,
        &include_reviews,
    )
    .fetch_all(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(books_list)))
}

pub async fn read_slug(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let book = sqlx::query_as!(
        Book,
        r#"SELECT
            b.title AS "title!",
            b.name AS "name!",
            b.summary AS "summary!",
            b.lang AS "language!",
            b.cover_image AS "cover_image!: _",
            b.spine_image AS "spine_image!: _",
            b.authors AS "authors!: _",
            b.categories AS "categories!: _",
            b.reactions AS "reactions?: _",
            coalesce(jsonb_agg(rv) FILTER (WHERE rv.id IS NOT NULL), '[]'::JSONB) AS "reviews!: sqlx::types::Json<Vec<Post>>"
        FROM books_view b
        LEFT JOIN LATERAL (
            SELECT *
            FROM fetch_posts(request_uid => $2) rv
            WHERE rv.book_id = b.id
            ORDER BY rv.id DESC
            LIMIT 5
            OFFSET 0
        ) rv ON TRUE
        WHERE b.name = $1
        GROUP BY b.title, b.name, b.summary, b.lang, b.cover_image, b.spine_image, b.authors, b.categories, b.reactions"#,
        &slug,
        &uid as &_
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => BooksError::BookNotFound(slug),
        _ => BooksError::Unexpected,
     })?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(book)))
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadCategoriesBook {
    title: String,
    name: String,
    cover_image: Option<AppImage>,
    spine_image: Option<AppImage>,
}
#[derive(serde::Serialize)]
pub struct ReadCategoriesResponse {
    id: i64,
    name: String,
    books: sqlx::types::Json<Vec<ReadCategoriesBook>>,
}

pub async fn read_categories(
    State(AppState { pool, .. }): State<AppState>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let categories = sqlx::query_as!(
        ReadCategoriesResponse,
        r#"SELECT
            bc.id AS "id!",
            bc.name AS "name!",
            b.books AS "books!: _"
        FROM book_categories bc
        LEFT JOIN LATERAL (
            SELECT coalesce(jsonb_agg(b ORDER BY b.rank DESC) FILTER (WHERE b.name IS NOT NULL), '[]'::JSONB) AS books
            FROM (
                SELECT btc.book_id
                FROM book_to_category btc
                WHERE btc.category_id = bc.id
            ) btc
            LEFT JOIN LATERAL (
                SELECT b.title, b.name, b.cover_image, b.spine_image, brt.rank
                FROM books_view b
                LEFT JOIN LATERAL (
                    SELECT books_boost_rating(brt) AS rank
                    FROM book_reactions_tally brt
                    WHERE brt.book_id = b.id
                ) brt ON TRUE
                WHERE b.id = btc.book_id
            ) b ON TRUE
        ) b ON TRUE
        GROUP BY bc.id, bc.name, b.books
        ORDER BY bc.id DESC"#,
    )
    .fetch_all(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(categories)))
}

pub async fn read_metadata(
    State(AppState { pool, .. }): State<AppState>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let metadata = sqlx::query_as!(
        BooksMetadata,
        r#"SELECT
            (
                SELECT coalesce(jsonb_agg(ba) FILTER (WHERE ba.id IS NOT NULL), '[]'::JSONB)
                FROM (SELECT id, name FROM book_authors) ba
            )
            AS "authors!: _",
            (
                SELECT coalesce(jsonb_agg(bc) FILTER (WHERE bc.id IS NOT NULL), '[]'::JSONB)
                FROM (SELECT id, name FROM book_categories) bc
            )
            AS "categories!: _"
        "#
    )
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(metadata)))
}

#[derive(serde::Deserialize, Validate)]
#[validate(schema(function = "validate_create_tracker_payload"))]
pub struct CreateTrackerPayload {
    #[validate(length(min = 0, message = "`book_name` must point to a valid book!"))]
    book_name: String,
    starts_at: NaiveDate,
    ends_at: NaiveDate,
}

fn validate_create_tracker_payload(payload: &CreateTrackerPayload) -> Result<(), ValidationError> {
    if payload.starts_at > payload.ends_at {
        return Err(ValidationError::new(
            "You cannot begin reading after finishing...",
        ));
    }
    if (payload.ends_at - payload.starts_at).num_days() > 365 {
        return Err(ValidationError::new(
            "Isn't that too much time to read a book?",
        ));
    }
    Ok(())
}

#[instrument(name = "Adding a reading tracker for user", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn create_tracker(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppForm(CreateTrackerPayload {
        book_name,
        starts_at,
        ends_at,
    }): AppForm<CreateTrackerPayload>,
) -> Result<Response, AppError> {
    let mut tx = pool.begin().await?;
    let book = sqlx::query!("SELECT id, title FROM books WHERE name = $1", &book_name)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::from(BooksError::BookNotFound(book_name.clone())),
            err => AppError::from(err),
        })?;
    sqlx::query!(
        "INSERT INTO users_books (user_id, book_id, starts_at, ends_at) VALUES ($1, $2, $3, $4)",
        &claims.sub,
        &book.id,
        &starts_at,
        &ends_at
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref err) => {
            if err.is_unique_violation() {
                AppError::from(BooksError::AlreadyTracking(book.title))
            } else {
                AppError::from(e)
            }
        }
        err => AppError::from(err),
    })?;
    tx.commit().await?;
    Ok(created(format!(
        "{}/books/tracking/{}",
        SETTINGS.frontend.url, book_name
    )))
}

#[derive(serde::Serialize)]
pub struct ReadingTracker {
    book_title: String,
    book_cover: Option<sqlx::types::Json<AppImage>>,
    book_spine: Option<sqlx::types::Json<AppImage>>,
    starts_at: NaiveDate,
    ends_at: NaiveDate,
    pages_read: i64,
    completed: bool,
}

#[instrument(name = "Fetching tracker...", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn fetch_tracker(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    Path(book): Path<String>,
) -> Result<Response, AppError> {
    let mut tx = pool.begin().await?;

    let tracker = sqlx::query_as!(
        ReadingTracker,
        r#"SELECT
            b.title AS "book_title!",
            b.cover_image AS "book_cover?: _",
            b.spine_image AS "book_spine?: _",
            ub.starts_at,
            ub.ends_at,
            ub.pages_read,
            ub.completed
        FROM users_books ub
        JOIN books_view b ON ub.book_id = b.id
        WHERE
            user_id = $1 AND
            b.name = $2 AND
            completed = FALSE"#,
        &claims.sub,
        &book
    )
    .fetch_one(&mut *tx)
    .await?;

    Ok(response(
        StatusCode::OK,
        None,
        AppJson(tracker),
    ))
}
