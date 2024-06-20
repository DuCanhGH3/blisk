-- Add up migration script here
CREATE TABLE IF NOT EXISTS books (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "summary" TEXT NOT NULL
);
