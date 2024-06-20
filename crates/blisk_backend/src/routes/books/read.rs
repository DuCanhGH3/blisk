use axum::{extract::State, http::StatusCode, response::Response};

use crate::{
    app::AppState,
    utils::{errors::AppError, json::AppJson, posts::structs::Post, response::response},
};

#[derive(serde::Serialize)]
struct Book {
    title: String,
    summary: String,
    reviews: sqlx::types::Json<Vec<Post>>,
}

pub async fn read(State(AppState { pool, .. }): State<AppState>) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let books_list = sqlx::query_as!(Book, 
        r#"SELECT 
            b.title,
            b.summary,
            COALESCE(JSON_AGG(rv) FILTER (WHERE rv IS NOT NULL), '[]'::JSON) AS "reviews!: sqlx::types::Json<Vec<Post>>" 
        FROM books b
        LEFT JOIN LATERAL (
            SELECT rv.title, rv.content, rvu.name as author_name, rv.reaction
            FROM posts rv
            JOIN users rvu ON rv.author_id = rvu.id
            WHERE rv.book_id = b.id
            ORDER BY rv.id DESC
            LIMIT 2
        ) rv ON TRUE
        GROUP BY b.id"#)
        .fetch_all(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(books_list)))
}
