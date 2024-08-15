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
  "text_search" TSVECTOR,
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

CREATE INDEX IF NOT EXISTS books_text_search_tsv ON books USING GIN ("text_search");

CREATE OR REPLACE FUNCTION after_update_books()
RETURNS trigger AS $trigger$
DECLARE
  authors TEXT;
  categories TEXT;
BEGIN
  SELECT immut_array_to_string(array_agg(ba."name")) INTO authors
  FROM book_authors ba
  JOIN book_to_author bta ON bta.author_id = ba.id
  WHERE bta.book_id = NEW.id
  GROUP BY bta.book_id;
  SELECT immut_array_to_string(array_agg(bc."name")) INTO categories
  FROM book_categories bc
  JOIN book_to_category btc ON btc.category_id = bc.id
  WHERE btc.book_id = NEW.id
  GROUP BY btc.book_id;
  UPDATE books SET text_search = (
    setweight(to_tsvector(coalesce("title", '')), 'A') || ' ' ||
    setweight(to_tsvector(coalesce("summary", '')), 'B') || ' ' ||
    setweight(to_tsvector(coalesce(authors, '')), 'B') || ' ' ||
    setweight(to_tsvector(coalesce(categories, '')), 'B') || ' ' ||
    setweight(to_tsvector(coalesce("name", '')), 'C')
  ) WHERE id = NEW.id;
  RETURN NEW;
END;
$trigger$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION after_link_to_book() RETURNS trigger SECURITY DEFINER AS
$trigger$
BEGIN
  -- Implementation trick to trigger recalculation of `text_search`.
  UPDATE books SET title = title WHERE id = NEW.book_id;
  RETURN NEW;
END;
$trigger$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_update_books
AFTER INSERT OR UPDATE OF "title", "summary", "name"
ON books FOR EACH ROW EXECUTE FUNCTION after_update_books();

CREATE OR REPLACE TRIGGER after_link_category_to_book
AFTER INSERT OR UPDATE ON book_to_category FOR EACH ROW EXECUTE FUNCTION after_link_to_book();

CREATE OR REPLACE TRIGGER after_link_author_to_book
AFTER INSERT OR UPDATE ON book_to_author FOR EACH ROW EXECUTE FUNCTION after_link_to_book();
