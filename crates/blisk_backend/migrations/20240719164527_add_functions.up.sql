-- Add up migration script here=
CREATE OR REPLACE FUNCTION fetch_replies(
  request_uid BIGINT,
  request_pid BIGINT,
  parent_id BIGINT,
  parent_path LTREE,
  current_level INT
)
RETURNS JSONB AS $$
DECLARE results JSONB;
BEGIN
  IF current_level = 0 THEN
    RETURN '[]'::JSONB;
  END IF;
  SELECT JSONB_AGG(rp) INTO results
  FROM (
    SELECT
      rp.id,
      rp.content,
      rp.post_id,
      u.id AS author_id,
      u.name AS author_name,
      u.picture AS author_picture,
      ucr.type AS user_reaction,
      fetch_replies(request_uid, request_pid, rp.id, rp.path, current_level - 1) AS children
    FROM comments rp
    JOIN users_view u
    ON u.id = rp.author_id
    LEFT JOIN comment_reactions ucr
    ON ucr.comment_id = rp.id AND ucr.user_id = request_uid
    WHERE rp.post_id = request_pid AND rp.path = parent_path || TEXT2LTREE(parent_id::TEXT)
    ORDER BY rp.id DESC
    LIMIT 5
  ) rp;
  RETURN results;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION fetch_posts(request_uid BIGINT)
RETURNS TABLE (
  id BIGINT,
  title TEXT,
  content TEXT,
  book_id BIGINT,
  author_name TEXT,
  author_picture JSONB,
  reaction BREACT,
  user_reaction PREACT
) AS $$
  SELECT
    rv.id,
    rv.title,
    rv.content,
    rv.book_id,
    rvu.name AS author_name,
    rvu.picture AS author_picture,
    rv.reaction,
    upr.type AS user_reaction
  FROM posts rv
  JOIN users_view rvu
  ON rv.author_id = rvu.id
  LEFT JOIN post_reactions upr
  ON upr.post_id = rv.id AND upr.user_id = request_uid;
$$
LANGUAGE sql;

CREATE OR REPLACE FUNCTION fetch_comments (
  request_uid BIGINT,
  replies_depth INT
)
RETURNS TABLE (
  id BIGINT,
  content TEXT,
  post_id BIGINT,
  path LTREE,
  author_id BIGINT,
  author_name TEXT,
  author_picture JSONB,
  user_reaction PREACT,
  children JSONB
) AS $$
  SELECT
    c.id,
    c.content,
    c.post_id,
    c.path,
    u.id AS author_id,
    u.name AS author_name,
    u.picture AS author_picture,
    ucr.type AS user_reaction,
    fetch_replies(
      request_uid => request_uid,
      request_pid => c.post_id,
      parent_id => c.id,
      parent_path => c.path,
      current_level => replies_depth
    ) AS children
  FROM comments c
  JOIN users_view u
  ON c.author_id = u.id
  LEFT JOIN comment_reactions ucr
  ON ucr.comment_id = c.id AND ucr.user_id = request_uid;
$$
LANGUAGE sql;

CREATE OR REPLACE FUNCTION create_post_reaction(
  rtype PREACT,
  usid BIGINT,
  pid BIGINT
)
RETURNS VOID AS $$
BEGIN
  PERFORM FROM post_reactions WHERE "user_id" = usid AND "post_id" = pid;
  IF FOUND THEN
    DELETE FROM post_reactions WHERE "user_id" = usid AND "post_id" = pid;        
  END IF;
  INSERT INTO post_reactions ("type", "user_id", "post_id") VALUES (rtype, usid, pid);
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION create_comment_reaction(
  rtype PREACT,
  usid BIGINT,
  cid BIGINT
)
RETURNS VOID AS $$
BEGIN
  PERFORM FROM comment_reactions WHERE "user_id" = usid AND "comment_id" = cid;
  IF FOUND THEN
    DELETE FROM comment_reactions WHERE "user_id" = usid AND "comment_id" = cid;        
  END IF;
  INSERT INTO comment_reactions ("type", "user_id", "comment_id") VALUES (rtype, usid, cid);
END;
$$
LANGUAGE plpgsql;