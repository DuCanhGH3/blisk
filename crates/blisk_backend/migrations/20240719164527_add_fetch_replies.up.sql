-- Add up migration script here
CREATE OR REPLACE FUNCTION fetch_replies(
    request_uid BIGINT,
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
            CASE
                WHEN current_level > 1 THEN fetch_replies(request_uid, rp.id, rp.path, current_level - 1)
            END AS children
        FROM comments rp
        JOIN users u
        ON u.id = rp.author_id
        LEFT JOIN comment_reactions ucr
        ON ucr.comment_id = rp.id AND ucr.user_id = request_uid
        WHERE rp.path = parent_path || TEXT2LTREE(parent_id::TEXT)
        ORDER BY rp.id DESC
        LIMIT 5
    ) rp;
    RETURN results;
END;
$$
LANGUAGE plpgsql;