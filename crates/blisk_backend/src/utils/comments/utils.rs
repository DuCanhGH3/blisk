use sqlx::Postgres;

use crate::utils::errors::AppError;

use super::structs::Comment;

struct CommentRaw {
    json_tree: sqlx::types::Json<Comment>,
}

pub async fn read<'c>(
    transaction: &mut sqlx::Transaction<'c, Postgres>,
    post_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<sqlx::types::Json<Comment>>, AppError> {
    let comments = sqlx::query_as!(
        CommentRaw,
        r#"WITH RECURSIVE c AS (
        	SELECT 
        		c.id, c.path, c.content, u.name AS author_name, nlevel(path) AS level, c.post_id
        	FROM
        		comments c
        	JOIN users u
        		ON c.author_id = u.id
        ), max_level AS (
        	SELECT MAX(level) max_level FROM c
        ), post_comments AS (
        	SELECT
        		c.*,
        		'[]'::JSON AS replies
        	FROM c, max_level
        	WHERE c.level = max_level
        	UNION ALL
        	SELECT
        		(c).*,
        		COALESCE(json_agg(rp) FILTER (WHERE rp IS NOT NULL), '[]'::JSON) as replies
        	FROM (
        		SELECT
                    c,
                    CASE
                        WHEN rp.path = c.path || TEXT2LTREE(c.id::TEXT) THEN rp
                        ELSE NULL
                    END AS rp
        		FROM c
        		JOIN post_comments rp
        		ON c.level = rp.level - 1
        	) AS comment
        	GROUP BY comment.c
        )

        SELECT ROW_TO_JSON(post_comments) AS "json_tree!: sqlx::types::Json<Comment>" 
        FROM post_comments 
        WHERE post_id = $1 AND path = 'Top'
        ORDER BY id DESC
        LIMIT $2
        OFFSET $3"#,
        &post_id,
        &limit,
        &offset,
    )
    .map(|e| e.json_tree)
    .fetch_all(&mut **transaction)
    .await?;
    Ok(comments)
}
