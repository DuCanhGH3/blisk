-- Add up migration script here
CREATE TABLE IF NOT EXISTS book_reactions_tally (
    "book_id" BIGINT PRIMARY KEY NOT NULL,
    "total" BIGINT NOT NULL DEFAULT 0,
    "like" BIGINT NOT NULL DEFAULT 0,
    "dislike" BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY ("book_id") REFERENCES books ("id") ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION calculate_book_reaction_delta(
    OLD posts,
    NEW posts,
    rtype BREACT
) RETURNS INT AS $$
    SELECT CASE
        -- INSERT
        WHEN OLD IS NULL AND NEW."reaction" = rtype THEN 1
        -- DELETE
        WHEN NEW IS NULL AND OLD."reaction" = rtype THEN -1
        -- UPDATE
        WHEN OLD."reaction" = rtype AND NEW."reaction" != rtype THEN -1
        -- UPDATE
        WHEN OLD."reaction" != rtype AND NEW."reaction" = rtype THEN 1
        -- ???
        ELSE 0
    END
$$
LANGUAGE sql;

CREATE OR REPLACE FUNCTION recalculate_books_reactions() RETURNS trigger SECURITY DEFINER AS
$trigger$
BEGIN
    -- An INSERT operation, insert new tally row if it doesn't already exist.
    IF OLD IS NULL THEN
        PERFORM FROM book_reactions_tally WHERE "book_id" = NEW."book_id";
        IF NOT FOUND THEN
            INSERT INTO book_reactions_tally ("book_id") VALUES (NEW."book_id");
        END IF;
    END IF;
    UPDATE book_reactions_tally SET
    -- Increase total if OLD is NULL (INSERT) and decrease if NEW is null (DELETE)
    "total" = "total" - CASE WHEN OLD IS NULL THEN -1 WHEN NEW IS NULL THEN 1 ELSE 0 END,
    "like" = "like" + calculate_book_reaction_delta(OLD, NEW, 'like'),
    "dislike" = "dislike" + calculate_book_reaction_delta(OLD, NEW, 'dislike')
    WHERE "book_id" = coalesce(NEW."book_id", OLD."book_id");
    RETURN coalesce(NEW, OLD);
END;
$trigger$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER recalculate_books_reactions
AFTER INSERT OR UPDATE OF "reaction" OR DELETE
ON posts FOR EACH ROW 
EXECUTE FUNCTION recalculate_books_reactions();
