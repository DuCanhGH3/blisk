-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "email" TEXT NOT NULL UNIQUE,
    "name" TEXT NOT NULL UNIQUE,
    "password" TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS users_id_email_is_active_idx ON users (id, email, is_active);
