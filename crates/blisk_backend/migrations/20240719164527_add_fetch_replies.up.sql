-- Add up migration script here
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
            u.name AS author_name,
            ucr.type AS user_reaction,
            fetch_replies(request_uid, request_pid, rp.id, rp.path, current_level - 1) AS children
        FROM comments rp
        JOIN users u
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