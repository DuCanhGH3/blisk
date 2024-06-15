-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "author_id" BIGSERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS posts_author_idx ON posts (author_id);
