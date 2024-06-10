-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts (
    "user_id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT NOT NULL
);