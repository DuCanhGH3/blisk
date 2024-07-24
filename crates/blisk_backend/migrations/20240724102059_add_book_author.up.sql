-- Add up migration script here
CREATE TABLE IF NOT EXISTS book_authors (
  "id" BIGSERIAL PRIMARY KEY,
  "name" TEXT
);

CREATE TABLE IF NOT EXISTS book_to_author (
  "book_id" BIGINT NOT NULL,
  "author_id" BIGINT NOT NULL,
  FOREIGN KEY ("book_id") REFERENCES books ("id") ON DELETE CASCADE,
  FOREIGN KEY ("author_id") REFERENCES book_authors ("id") ON DELETE CASCADE,
  PRIMARY KEY ("book_id", "author_id")
);