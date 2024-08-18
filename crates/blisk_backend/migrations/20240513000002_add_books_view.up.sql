-- Add up migration script here
CREATE OR REPLACE VIEW books_view AS (
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
    bc.bc_json AS categories,
    bc.bc_raw AS categories_raw,
    ba.ba_json AS authors,
    ba.ba_raw AS authors_raw
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
  LEFT JOIN LATERAL (
    SELECT
      COALESCE(JSONB_AGG(bc) FILTER (WHERE bc.id IS NOT NULL), '[]'::JSONB) AS bc_json,
      COALESCE(ARRAY_AGG(bc.id) FILTER (WHERE bc.id IS NOT NULL), '{}'::BIGINT[]) AS bc_raw
    FROM (
      SELECT bc.id, bc.name
      FROM book_to_category btc
      JOIN book_categories bc
      ON bc.id = btc.category_id
      WHERE btc.book_id = b.id
    ) bc
  ) bc ON TRUE
  LEFT JOIN LATERAL (
    SELECT
      COALESCE(JSONB_AGG(ba) FILTER (WHERE ba.id IS NOT NULL), '[]'::JSONB) AS ba_json,
      COALESCE(ARRAY_AGG(ba.id) FILTER (WHERE ba.id IS NOT NULL), '{}'::BIGINT[]) AS ba_raw
    FROM (
      SELECT ba.id, ba.name
      FROM book_to_author bta
      JOIN book_authors ba
      ON ba.id = bta.author_id
      WHERE bta.book_id = b.id 
    ) ba
  ) ba ON TRUE
  GROUP BY b.id, bl.name, bl.code, ci.id, ci.ext, ci.owner_id, si.id,
  si.ext, si.owner_id, bc.bc_json, bc.bc_raw, ba.ba_json, ba.ba_raw
);