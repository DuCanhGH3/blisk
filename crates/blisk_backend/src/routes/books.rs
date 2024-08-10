use super::{auth::OptionalUserClaims, posts::Post};
use crate::{
    app::AppState,
    utils::{
        constants::SLUG_REGEX,
        errors::{AppError, BooksError},
        response::response,
        structs::AppJson,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use validator::Validate;

#[derive(serde::Deserialize, Validate)]
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
    #[validate(length(min = 1, message = "Book must has at least one category!"))]
    categories: Vec<i64>,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
    AppJson(CreatePayload {
        title,
        slug,
        pages,
        language,
        summary,
        categories,
    }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let bid: i64 = sqlx::query_scalar!(
        "INSERT INTO books (title, name, pages, language, summary) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        &title,
        &slug,
        &pages,
        &language,
        &summary
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(ref db_err) => match db_err.constraint() {
            Some("books_name_key") => AppError::from(BooksError::SlugAlreadyExists(slug)),
            Some("books_language_fkey") => AppError::from(BooksError::LanguageInvalid(language)),
            _ => AppError::from(err)
        },
        err => AppError::from(err),
    })?;
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
    summary: String,
    authors: sqlx::types::Json<Vec<BookAuthor>>,
    categories: sqlx::types::Json<Vec<BookCategory>>,
    reviews: sqlx::types::Json<Vec<Post>>,
}

pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
) -> Result<Response, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let books_list = sqlx::query_as!(
        Book,
        r#"SELECT 
            b.title AS "title!: String",
            b.summary AS "summary!: String",
            b.authors AS "authors!: sqlx::types::Json<Vec<BookAuthor>>",
            b.categories AS "categories!: sqlx::types::Json<Vec<BookCategory>>",
            COALESCE(JSONB_AGG(rv) FILTER (WHERE rv.id IS NOT NULL), '[]'::JSONB) AS "reviews!: sqlx::types::Json<Vec<Post>>"
        FROM book_view b
        LEFT JOIN LATERAL (
            SELECT *
            FROM fetch_posts(request_uid => $1) rv
            WHERE rv.book_id = b.id
            ORDER BY rv.id DESC
            LIMIT 5
            OFFSET 0
        ) rv ON TRUE
        GROUP BY b.id, b.title, b.summary, b.authors, b.categories"#,
        &uid as &_
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
            b.title AS "title!: String",
            b.summary AS "summary!: String",
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
        GROUP BY b.id, b.title, b.summary, b.authors, b.categories"#,
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
