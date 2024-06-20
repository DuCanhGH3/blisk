-- Add up migration script here
CREATE TYPE BREACT AS ENUM (
    'like',
    'dislike'
);

CREATE TABLE IF NOT EXISTS posts (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "author_id" BIGINT NOT NULL,
    "book_id" BIGINT NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "reaction" BREACT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES books (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS posts_author_book_idx ON posts (author_id, book_id);
