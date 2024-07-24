-- Add up migration script here
CREATE OR REPLACE VIEW book_view AS (
  SELECT
    b.id,
    b.title,
    b.summary,
    (
      SELECT COALESCE(JSONB_AGG(bc) FILTER (WHERE bc.id IS NOT NULL), '[]'::JSONB)
      FROM (
        SELECT bc.id, bc.name
        FROM book_to_category btc
        JOIN book_categories bc
        ON bc.id = btc.category_id
        WHERE btc.book_id = b.id
      ) bc
    ) AS categories,
    (
      SELECT COALESCE(JSONB_AGG(ba) FILTER (WHERE ba.id IS NOT NULL), '[]'::JSONB)
      FROM (
        SELECT ba.id, ba.name
        FROM book_to_author bta
        JOIN book_authors ba
        ON ba.id = bta.author_id
        WHERE bta.book_id = b.id 
      ) ba
    ) AS authors
  FROM books b
  GROUP BY b.id
);