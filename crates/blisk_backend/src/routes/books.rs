use super::{auth::OptionalUserClaims, posts::Post};
use crate::{
    app::AppState,
    utils::{
        errors::AppError,
        response::response,
        structs::{AppJson, AppMultipart},
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use axum_typed_multipart::TryFromMultipart;

#[derive(TryFromMultipart)]
pub struct CreatePayload {
    title: String,
    summary: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
    AppMultipart(CreatePayload { title, summary }): AppMultipart<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let bid: i64 = sqlx::query_scalar!(
        "INSERT INTO books (title, summary) VALUES ($1, $2) RETURNING id",
        &title,
        &summary
    )
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(CreateResponse { id: bid }),
    ))
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct Book {
    title: String,
    summary: String,
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
        let book: Book = sqlx::query_as(r#"
            SELECT 
                b.title,
                b.summary,
                COALESCE(JSONB_AGG(rv), '[]'::JSONB) AS reviews
            FROM books b, LATERAL (
                SELECT * FROM fetch_posts(
                    request_uid => $2,
                    request_limit => 5,
                    request_offset => 0
                )
            ) rv
            WHERE b.id = $1
            GROUP BY b.id
        "#)
        .bind(&bid)
        .bind(&uid)
        .fetch_one(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(response(StatusCode::OK, None, AppJson(book)))
    } else {
        let mut transaction = pool.begin().await?;
        let books_list: Vec<Book> = sqlx::query_as(r#"
            SELECT 
                b.title,
                b.summary,
                COALESCE(JSONB_AGG(rv), '[]'::JSONB) AS reviews
            FROM books b, LATERAL (
                SELECT * FROM fetch_posts(
                    request_uid => $1,
                    request_limit => 5,
                    request_offset => 0
                )
            ) rv
            GROUP BY b.id
        "#)
        .bind(&uid)
        .fetch_all(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(response(StatusCode::OK, None, AppJson(books_list)))
    }
}
