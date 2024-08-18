use super::{
    auth::{OptionalUserClaims, UserClaims},
    posts::Post,
};
use crate::{
    app::AppState,
    utils::{
        constants::SLUG_REGEX,
        errors::AppError,
        response::response,
        structs::{AppImage, AppJson, AppMultipart, AppQuery},
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
use tracing::instrument;
use validator::Validate;

#[derive(Debug, thiserror::Error)]
pub enum BooksError {
    #[error("duplicate slug {0}")]
    SlugAlreadyExists(String),
    #[error("language {0} does not exist")]
    LanguageInvalid(String),
    #[error("book {0} cannot be found")]
    BookNotFound(String),
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
    /// The cover image of the book.
    cover_image: FieldData<Bytes>,
    /// The spine image of the book.
    spine_image: FieldData<Bytes>,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

#[instrument(name = "Creating a new book...", skip(pool, claims, cover_image, spine_image), fields(uid = %claims.sub))]
pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
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
    let cover_id = upload_file(&mut transaction, claims.sub, None, cover_image).await?;
    let spine_id = upload_file(&mut transaction, claims.sub, None, spine_image).await?;
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
            Some("books_name_key") => AppError::from(BooksError::SlugAlreadyExists(slug)),
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
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(CreateResponse { id: bid }),
    ))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BookAuthor {
    id: i64,
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BookCategory {
    id: i64,
    name: String,
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct Book {
    title: String,
    name: String,
    summary: String,
    cover_image: sqlx::types::Json<AppImage>,
    spine_image: sqlx::types::Json<AppImage>,
    authors: sqlx::types::Json<Vec<BookAuthor>>,
    categories: sqlx::types::Json<Vec<BookCategory>>,
    reviews: Option<sqlx::types::Json<Vec<Post>>>,
}

#[derive(serde::Deserialize, Validate)]
pub struct ReadQuery {
    #[validate(length(min = 1, message = "Query is not valid!"))]
    q: Option<String>,
    #[validate(range(min = 0, message = "Offset is not valid!"))]
    offset: Option<i64>,
    include_reviews: Option<bool>,
}

pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    AppQuery(ReadQuery {
        q,
        offset,
        include_reviews,
    }): AppQuery<ReadQuery>,
) -> Result<Response, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let include_reviews = include_reviews.unwrap_or(true);
    let books_list = if let Some(query) = q {
        let offset = offset.unwrap_or(0);
        sqlx::query_as!(
            Book,
            r#"
            SELECT
                b.title AS "title!",
                b.name AS "name!",
                b.summary AS "summary!",
                b.cover_image AS "cover_image!: _",
                b.spine_image AS "spine_image!: _",
                b.authors AS "authors!: _",
                b.categories AS "categories!: _",
                CASE $4::BOOLEAN
                    WHEN TRUE THEN COALESCE(JSONB_AGG(rv) FILTER (WHERE rv.id IS NOT NULL), '[]'::JSONB)
                    ELSE NULL
                END AS "reviews?: _"
            FROM book_view b, websearch_to_tsquery($2) query, ts_rank(b.text_search, query) rank
            LEFT JOIN LATERAL (
                SELECT *
                FROM fetch_posts(request_uid => $1) rv
                WHERE rv.book_id = b.id
                ORDER BY rv.id DESC
                LIMIT 5
                OFFSET 0
            ) rv ON TRUE
            WHERE b.text_search @@ query
            GROUP BY b.title, b.name, b.summary, b.cover_image, b.spine_image, b.authors, b.categories, rank.rank
            ORDER BY rank DESC
            LIMIT 20
            OFFSET $3"#,
            &uid as &_,
            &query,
            &offset,
            &include_reviews,
        ).fetch_all(&mut *transaction).await?
    } else {
        sqlx::query_as!(
            Book,
            r#"SELECT 
                b.title AS "title!: String",
                b.name AS "name!: String",
                b.summary AS "summary!: String",
                b.cover_image AS "cover_image!: _",
                b.spine_image AS "spine_image!: _",
                b.authors AS "authors!: sqlx::types::Json<Vec<BookAuthor>>",
                b.categories AS "categories!: sqlx::types::Json<Vec<BookCategory>>",
                CASE $2::BOOLEAN
                    WHEN TRUE THEN COALESCE(JSONB_AGG(rv) FILTER (WHERE rv.id IS NOT NULL), '[]'::JSONB)
                    ELSE NULL
                END AS "reviews?: _"
            FROM book_view b
            LEFT JOIN LATERAL (
                SELECT *
                FROM fetch_posts(request_uid => $1) rv
                WHERE rv.book_id = b.id
                ORDER BY rv.id DESC
                LIMIT 5
                OFFSET 0
            ) rv ON TRUE
            GROUP BY b.title, b.name, b.summary, b.cover_image, b.spine_image, b.authors, b.categories"#,
            &uid as &_,
            &include_reviews,
        ).fetch_all(&mut *transaction).await?
    };
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
            b.title AS "title!: String",
            b.name AS "name!: String",
            b.summary AS "summary!: String",
            b.cover_image AS "cover_image!: _",
            b.spine_image AS "spine_image!: _",
            b.authors AS "authors!: sqlx::types::Json<Vec<BookAuthor>>",
            b.categories AS "categories!: sqlx::types::Json<Vec<BookCategory>>",
            COALESCE(JSONB_AGG(rv) FILTER (WHERE rv.id IS NOT NULL), '[]'::JSONB) AS "reviews!: sqlx::types::Json<Vec<Post>>"
        FROM book_view b
        LEFT JOIN LATERAL (
            SELECT *
            FROM fetch_posts(request_uid => $2) rv
            WHERE rv.book_id = b.id
            ORDER BY rv.id DESC
            LIMIT 5
            OFFSET 0
        ) rv ON TRUE
        WHERE b.name = $1
        GROUP BY b.title, b.name, b.summary, b.cover_image, b.spine_image, b.authors, b.categories"#,
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

pub async fn read_categories(
    State(AppState { pool, .. }): State<AppState>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let categories = sqlx::query_as!(
        BookCategory,
        r#"SELECT id AS "id!: _", name AS "name!: _" FROM book_categories"#
    )
    .fetch_all(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(categories)))
}
