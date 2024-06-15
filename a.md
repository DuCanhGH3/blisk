WITH RECURSIVE c AS (
	SELECT 
		c.id, c.path, c.content, u.name AS author_name, nlevel(path) AS level
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
		COALESCE(json_agg(rp), '[]'::JSON) as replies
	FROM (
		SELECT c, rp
		FROM c
		JOIN post_comments rp 
		ON rp.path = c.path || TEXT2LTREE(c.id::TEXT)
	) AS comment
	GROUP BY comment.c
)

SELECT ROW_TO_JSON(post_comments) AS json_tree FROM post_comments WHERE path = text2ltree('Top'::VARCHAR(255));

WITH post_comments AS (
	WITH base_comments AS (
		SELECT 
			c.id, c.path, c.content, u.name as author_name
		FROM 
			comments c
		JOIN users u
			ON c.author_id = u.id
		WHERE path = text2ltree('Top'::VARCHAR(255))
		LIMIT 20
		OFFSET 0
	), base_replies AS (
		SELECT 
			rp.id, rp.path, rp.content, u.name as author_name
		FROM 
			comments rp
		JOIN users u 
			ON rp.author_id = u.id
		WHERE rp.path <@ (
			SELECT path || text2ltree(id::VARCHAR(255))
			FROM base_comments
		)
	)
	SELECT * FROM base_comments
	UNION ALL
	SELECT * FROM base_replies
)

SELECT * FROM post_comments ORDER BY path;