-- Add up migration script here
CREATE TABLE IF NOT EXISTS book_languages (
  "code" TEXT NOT NULL PRIMARY KEY,
  "name" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS book_categories (
  "id" BIGSERIAL PRIMARY KEY,
  "name" TEXT
);

CREATE TABLE IF NOT EXISTS book_authors (
  "id" BIGSERIAL PRIMARY KEY,
  "name" TEXT
);

CREATE TABLE IF NOT EXISTS books (
  "id" BIGSERIAL NOT NULL PRIMARY KEY,
  "title" TEXT NOT NULL,
  "name" TEXT NOT NULL UNIQUE,
  "pages" INT NOT NULL,
  "language" TEXT NOT NULL,
  "summary" TEXT NOT NULL,
  FOREIGN KEY ("language") REFERENCES book_languages ("code") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS book_to_category (
  "book_id" BIGINT NOT NULL,
  "category_id" BIGINT NOT NULL,
  FOREIGN KEY ("book_id") REFERENCES books ("id") ON DELETE CASCADE,
  FOREIGN KEY ("category_id") REFERENCES book_categories ("id") ON DELETE CASCADE,
  PRIMARY KEY ("book_id", "category_id")
);

CREATE TABLE IF NOT EXISTS book_to_author (
  "book_id" BIGINT NOT NULL,
  "author_id" BIGINT NOT NULL,
  FOREIGN KEY ("book_id") REFERENCES books ("id") ON DELETE CASCADE,
  FOREIGN KEY ("author_id") REFERENCES book_authors ("id") ON DELETE CASCADE,
  PRIMARY KEY ("book_id", "author_id")
);

CREATE INDEX IF NOT EXISTS fkey_books_book_languages ON books ("language");
