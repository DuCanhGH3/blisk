use sqlx::Postgres;

use crate::utils::errors::AppError;

use super::structs::Comment;

pub async fn read<'c>(
    transaction: &mut sqlx::Transaction<'c, Postgres>,
    post_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Comment>, AppError> {
    let comments = sqlx::query_as!(
        Comment,
        r#"SELECT
            c.id,
            c.content,
            u.name AS author_name,
            COALESCE(JSON_AGG(rp) FILTER (WHERE rp IS NOT NULL), '[]'::JSON) as "replies!: sqlx::types::Json<Vec<Comment>>"
        FROM comments c
        JOIN users u
        ON c.author_id = u.id
        LEFT JOIN LATERAL (
            SELECT
                rp.id,
                rp.content,
                urp.name as author_name
            FROM comments rp
            JOIN users urp
            ON rp.author_id = urp.id
            WHERE rp.path = c.path || TEXT2LTREE(c.id::TEXT)
            ORDER BY rp.id DESC
            LIMIT 5
        ) rp ON true
        WHERE c.post_id = $1 AND c.path = 'Top'
        GROUP BY c.id, u.name
        ORDER BY c.id DESC
        LIMIT $2
        OFFSET $3"#,
        &post_id,
        &limit,
        &offset,
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(comments)
}
