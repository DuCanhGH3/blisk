-- Add up migration script here
-- CREATE TYPE PREACT AS ENUM (
--     'like',
--     'love',
--     'laugh',
--     'wow',
--     'sad',
--     'angry'
-- );

CREATE TABLE IF NOT EXISTS post_reactions (
    "type" PREACT NOT NULL,
    "user_id" BIGINT NOT NULL,
    "post_id" BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, post_id)
);

CREATE TABLE IF NOT EXISTS comment_reactions (
    "type" PREACT NOT NULL,
    "user_id" BIGINT NOT NULL,
    "comment_id" BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (comment_id) REFERENCES comments (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, comment_id)
);

CREATE TABLE IF NOT EXISTS post_reactions_tally (
    "post_id" BIGINT PRIMARY KEY NOT NULL,
    "total" BIGINT NOT NULL DEFAULT 0,
    "like" BIGINT NOT NULL DEFAULT 0,
    "love" BIGINT NOT NULL DEFAULT 0,
    "laugh" BIGINT NOT NULL DEFAULT 0,
    "wow" BIGINT NOT NULL DEFAULT 0,
    "sad" BIGINT NOT NULL DEFAULT 0,
    "angry" BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY ("post_id") REFERENCES posts ("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS comment_reactions_tally (
    "comment_id" BIGINT PRIMARY KEY NOT NULL,
    "total" BIGINT NOT NULL DEFAULT 0,
    "like" BIGINT NOT NULL DEFAULT 0,
    "love" BIGINT NOT NULL DEFAULT 0,
    "laugh" BIGINT NOT NULL DEFAULT 0,
    "wow" BIGINT NOT NULL DEFAULT 0,
    "sad" BIGINT NOT NULL DEFAULT 0,
    "angry" BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY ("comment_id") REFERENCES comments ("id") ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION calculate_reaction_delta(
    OLD ANYELEMENT,
    NEW ANYELEMENT,
    rtype PREACT
) RETURNS INT AS $$
    SELECT CASE
        -- INSERT
        WHEN OLD IS NULL AND NEW."type" = rtype THEN 1
        -- DELETE
        WHEN NEW IS NULL AND OLD."type" = rtype THEN -1
        -- UPDATE
        WHEN OLD."type" = rtype AND NEW."type" != rtype THEN -1
        -- UPDATE
        WHEN OLD."type" != rtype AND NEW."type" = rtype THEN 1
        -- ???
        ELSE 0
    END
$$
LANGUAGE sql;

CREATE OR REPLACE FUNCTION recalculate_posts_reactions() RETURNS trigger SECURITY DEFINER AS
$trigger$
BEGIN
    -- An INSERT operation, insert new tally row if it doesn't already exist.
    IF OLD IS NULL THEN
        PERFORM FROM post_reactions_tally WHERE "post_id" = NEW."post_id";
        IF NOT FOUND THEN
            INSERT INTO post_reactions_tally ("post_id") VALUES (NEW."post_id");
        END IF;
    END IF;
    UPDATE post_reactions_tally SET
    -- Increase total if OLD is NULL (INSERT) and decrease if NEW is null (DELETE)
    "total" = "total" - CASE WHEN OLD IS NULL THEN -1 WHEN NEW IS NULL THEN 1 ELSE 0 END,
    "like" = "like" + calculate_reaction_delta(OLD, NEW, 'like'),
    "love" = "love" + calculate_reaction_delta(OLD, NEW, 'love'),
    "laugh" = "laugh" + calculate_reaction_delta(OLD, NEW, 'laugh'),
    "wow" = "wow" + calculate_reaction_delta(OLD, NEW, 'wow'),
    "sad" = "sad" + calculate_reaction_delta(OLD, NEW, 'sad'),
    "angry" = "angry" + calculate_reaction_delta(OLD, NEW, 'angry')
    WHERE "post_id" = coalesce(NEW."post_id", OLD."post_id");
    RETURN coalesce(NEW, OLD);
END;
$trigger$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION recalculate_comment_reactions() RETURNS trigger SECURITY DEFINER AS
$trigger$
BEGIN
    -- An INSERT operation, insert new tally row if it doesn't already exist.
    IF OLD IS NULL THEN
        PERFORM FROM comment_reactions_tally WHERE "comment_id" = NEW."comment_id";
        IF NOT FOUND THEN
            INSERT INTO comment_reactions_tally ("comment_id") VALUES (NEW."comment_id");
        END IF;
    END IF;
    UPDATE comment_reactions_tally SET
    -- Increase total if OLD is NULL (INSERT) or decrease if NEW is null (DELETE)
    "total" = "total" - CASE WHEN OLD IS NULL THEN -1 WHEN NEW IS NULL THEN 1 ELSE 0 END,
    "like" = "like" + calculate_reaction_delta(OLD, NEW, 'like'),
    "love" = "love" + calculate_reaction_delta(OLD, NEW, 'love'),
    "laugh" = "laugh" + calculate_reaction_delta(OLD, NEW, 'laugh'),
    "wow" = "wow" + calculate_reaction_delta(OLD, NEW, 'wow'),
    "sad" = "sad" + calculate_reaction_delta(OLD, NEW, 'sad'),
    "angry" = "angry" + calculate_reaction_delta(OLD, NEW, 'angry')
    WHERE "comment_id" = coalesce(NEW."comment_id", OLD."comment_id");
    RETURN coalesce(NEW, OLD);
END;
$trigger$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER recalculate_posts_reactions
AFTER INSERT OR UPDATE OF "type" OR DELETE
ON post_reactions FOR EACH ROW 
EXECUTE FUNCTION recalculate_posts_reactions();

CREATE OR REPLACE TRIGGER recalculate_comment_reactions
AFTER INSERT OR UPDATE OF "type" OR DELETE
ON comment_reactions FOR EACH ROW 
EXECUTE FUNCTION recalculate_comment_reactions();