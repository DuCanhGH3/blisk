-- Add up migration script here
CREATE OR REPLACE FUNCTION fetch_posts(
  request_uid BIGINT,
  request_limit NUMERIC,
  request_offset NUMERIC
)
RETURNS TABLE (
  id BIGINT,
  title TEXT,
  content TEXT,
  book_id BIGINT,
  author_name TEXT,
  reaction BREACT,
  user_reaction PREACT
) AS $$
BEGIN
  RETURN QUERY
  SELECT
    rv.id,
    rv.title,
    rv.content,
    rv.book_id,
    rvu.name AS author_name,
    rv.reaction,
    upr.type AS user_reaction
  FROM posts rv
  JOIN users rvu
  ON rv.author_id = rvu.id
  LEFT JOIN post_reactions upr
  ON upr.post_id = rv.id AND upr.user_id = request_uid
  ORDER BY rv.id DESC
  LIMIT request_limit
  OFFSET request_offset;
END;
$$
LANGUAGE plpgsql;