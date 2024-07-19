-- Add up migration script here
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