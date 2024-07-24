use super::{auth::OptionalUserClaims, posts::Post};
use crate::{
    app::AppState,
    utils::{
        errors::{AppError, BooksError},
        response::response,
        structs::AppJson,
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    title: String,
    summary: String,
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
        summary,
        categories,
    }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let bid: i64 = sqlx::query_scalar!(
        "INSERT INTO books (title, summary) VALUES ($1, $2) RETURNING id",
        &title,
        &summary
    )
    .fetch_one(&mut *transaction)
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
    summary: String,
    authors: sqlx::types::Json<Vec<BookAuthor>>,
    categories: sqlx::types::Json<Vec<BookCategory>>,
    reviews: sqlx::types::Json<Vec<Post>>,
}

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    book_id: Option<i64>,
}

pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    Query(ReadQuery { book_id }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);
    if let Some(bid) = book_id {
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
                WHERE rv.book_id = $1
                ORDER BY rv.id DESC
                LIMIT 5
                OFFSET 0
            ) rv ON TRUE
            WHERE b.id = $1
            GROUP BY b.id, b.title, b.summary, b.authors, b.categories"#,
            &bid,
            &uid as &_
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BooksError::BookNotFound(bid),
            _ => BooksError::Unexpected,
         })?;
        transaction.commit().await?;
        Ok(response(StatusCode::OK, None, AppJson(book)))
    } else {
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
}
