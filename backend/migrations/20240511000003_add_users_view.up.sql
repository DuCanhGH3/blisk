-- Add up migration script here
CREATE OR REPLACE VIEW users_view AS (
  SELECT
    u."id", u."email", u."name", u."role",
    construct_image(pfp."owner_id", pfp."id", pfp."ext") AS picture,
    u."is_verified", u."password"
  FROM users u
  LEFT JOIN LATERAL (
    SELECT f."id", f."owner_id", f."ext"
    FROM files f WHERE f."id" = u."picture_id"
  ) pfp ON TRUE
);