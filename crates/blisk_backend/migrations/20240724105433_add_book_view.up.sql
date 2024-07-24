-- Add up migration script here
CREATE OR REPLACE VIEW book_view AS (
  SELECT
    b.id,
    b.title,
    b.summary,
    COALESCE(JSONB_AGG(bc) FILTER (WHERE bc.id IS NOT NULL), '[]'::JSONB) AS categories
  FROM books b
  LEFT JOIN LATERAL (
    SELECT bc.id, bc.name
    FROM book_to_category btc
    JOIN book_categories bc
    ON bc.id = btc.category_id
    WHERE btc.book_id = b.id
  ) bc ON TRUE
  GROUP BY b.id
);