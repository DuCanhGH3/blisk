-- Add up migration script here
CREATE OR REPLACE FUNCTION fetch_comments (
  request_uid BIGINT,
  request_pid BIGINT,
  replies_depth INT
)
RETURNS TABLE (
  id BIGINT,
  content TEXT,
  author_name TEXT,
  user_reaction PREACT,
  children JSONB
) AS $$
BEGIN
  RETURN QUERY
  SELECT
    c.id,
    c.content,
    u.name AS author_name,
    ucr.type AS user_reaction,
    fetch_replies(
      request_uid => request_uid,
      request_pid => request_pid,
      parent_id => c.id,
      parent_path => c.path,
      current_level => replies_depth
    ) AS children
  FROM comments c
  JOIN users u
  ON c.author_id = u.id
  LEFT JOIN comment_reactions ucr
  ON ucr.comment_id = c.id AND ucr.user_id = request_uid
  WHERE c.post_id = request_pid AND c.path = 'Top';
END;
$$
LANGUAGE plpgsql;