-- Add up migration script here
CREATE TYPE ROLE AS ENUM ('admin', 'user');

CREATE TABLE IF NOT EXISTS users (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "email" TEXT NOT NULL UNIQUE,
    "name" TEXT NOT NULL UNIQUE,
    "is_verified" BOOLEAN NOT NULL DEFAULT FALSE,
    "role" ROLE NOT NULL DEFAULT 'user',
    "password" TEXT NOT NULL
);
