-- Add down migration script here
DROP INDEX IF EXISTS users_books_completed_idx;

DROP TABLE IF EXISTS users_books;