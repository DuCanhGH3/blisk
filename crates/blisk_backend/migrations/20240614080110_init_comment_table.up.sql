-- Add up migration script here
CREATE TABLE IF NOT EXISTS comments (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "post_id" BIGSERIAL NOT NULL,
    "author_id" BIGSERIAL NOT NULL,
    "content" TEXT NOT NULL,
    "path" LTREE NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS comments_posts_author_idx ON comments (post_id, author_id);
