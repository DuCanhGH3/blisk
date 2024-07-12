use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};

use crate::{
    app::AppState,
    utils::{errors::AppError, posts::structs::Post, response::response, structs::AppJson},
};

#[derive(serde::Serialize)]
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
    Query(ReadQuery { book_id }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    if let Some(bid) = book_id {
        let mut transaction = pool.begin().await?;
        let book = sqlx::query_as!(Book, r#"
            SELECT 
                b.title,
                b.summary,
                COALESCE(JSON_AGG(rv) FILTER (WHERE rv IS NOT NULL), '[]'::JSON) AS "reviews!: sqlx::types::Json<Vec<Post>>" 
            FROM books b
            LEFT JOIN LATERAL (
                SELECT rv.id, rv.title, rv.content, rvu.name as author_name, rv.reaction
                FROM posts rv
                JOIN users rvu ON rv.author_id = rvu.id
                WHERE rv.book_id = b.id
                ORDER BY rv.id DESC
                LIMIT 2
            ) rv ON TRUE
            WHERE b.id = $1
            GROUP BY b.id
        "#, &bid)
        .fetch_one(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(response(StatusCode::OK, None, AppJson(book)))
    } else {
        let mut transaction = pool.begin().await?;
        let books_list = sqlx::query_as!(Book, r#"
            SELECT 
                b.title,
                b.summary,
                COALESCE(JSON_AGG(rv) FILTER (WHERE rv IS NOT NULL), '[]'::JSON) AS "reviews!: sqlx::types::Json<Vec<Post>>" 
            FROM books b
            LEFT JOIN LATERAL (
                SELECT rv.id, rv.title, rv.content, rvu.name as author_name, rv.reaction
                FROM posts rv
                JOIN users rvu ON rv.author_id = rvu.id
                WHERE rv.book_id = b.id
                ORDER BY rv.id DESC
                LIMIT 2
            ) rv ON TRUE
            GROUP BY b.id
        "#)
        .fetch_all(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(response(StatusCode::OK, None, AppJson(books_list)))
    }
}
