-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "user_id" BIGSERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);