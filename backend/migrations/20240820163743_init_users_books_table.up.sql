-- Add up migration script here
CREATE TABLE IF NOT EXISTS users_books (
  "user_id" BIGINT NOT NULL,
  "book_id" BIGINT NOT NULL,
  "starts_at" DATE NOT NULL,
  "ends_at" DATE NOT NULL,
  "pages_read" BIGINT NOT NULL DEFAULT 0,
  "completed" BOOLEAN NOT NULL DEFAULT FALSE,
  FOREIGN KEY ("user_id") REFERENCES users ("id") ON DELETE CASCADE,
  FOREIGN KEY ("book_id") REFERENCES books ("id") ON DELETE CASCADE,
  -- Allow users to re-read a book, but don't allow them to initiate
  -- a new tracker while another tracker is still active.
  PRIMARY KEY ("user_id", "book_id", "completed")
);

CREATE INDEX IF NOT EXISTS users_books_completed_idx ON users_books ("completed");