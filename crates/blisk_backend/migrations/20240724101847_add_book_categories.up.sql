-- Add up migration script here
CREATE TABLE IF NOT EXISTS book_categories (
  "id" BIGSERIAL PRIMARY KEY,
  "name" TEXT
);

CREATE TABLE IF NOT EXISTS book_to_category (
  "book_id" BIGINT NOT NULL,
  "category_id" BIGINT NOT NULL,
  FOREIGN KEY ("book_id") REFERENCES books ("id") ON DELETE CASCADE,
  FOREIGN KEY ("category_id") REFERENCES book_categories ("id") ON DELETE CASCADE,
  PRIMARY KEY ("book_id", "category_id")
);