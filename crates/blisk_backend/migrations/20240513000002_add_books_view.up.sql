-- Add up migration script here
CREATE OR REPLACE VIEW book_view AS (
  SELECT
    b.id,
    b.title,
    b.name,
    b.pages,
    b.summary,
    construct_image(ci.owner_id, ci.id, ci.ext) AS cover_image,
    construct_image(si.owner_id, si.id, si.ext) AS spine_image,
    b.text_search,
    bl.name AS lang,
    bl.code AS lang_code,
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
  JOIN book_languages bl ON b.language = bl.code
  LEFT JOIN LATERAL (
    SELECT id, owner_id, ext
    FROM files WHERE id = b.cover_id
  ) ci ON TRUE
  LEFT JOIN LATERAL (
    SELECT id, owner_id, ext
    FROM files WHERE id = b.spine_id
  ) si ON TRUE
  GROUP BY b.id, bl.name, bl.code, ci.id, ci.ext, ci.owner_id, si.id, si.ext, si.owner_id
);