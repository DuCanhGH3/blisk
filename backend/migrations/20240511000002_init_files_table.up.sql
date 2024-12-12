-- Add up migration script here
CREATE TABLE IF NOT EXISTS files (
    "id" BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    "owner_id" BIGINT NOT NULL,
    "parent_id" BIGINT DEFAULT NULL,
    "ext" TEXT NOT NULL,
    "path" LTREE DEFAULT 'Top',
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE SET DEFAULT,
    FOREIGN KEY (parent_id) REFERENCES files (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS files_owner_id_parent_id_idx ON files (owner_id, parent_id);

ALTER TABLE users ADD COLUMN "picture_id" BIGINT REFERENCES files ("id") ON DELETE SET NULL;