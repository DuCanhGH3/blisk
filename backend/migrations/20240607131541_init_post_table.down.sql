-- Add down migration script here
DROP INDEX IF EXISTS posts_author_book_idx;

DROP TABLE IF EXISTS posts;

DROP TYPE BREACT;