-- Add down migration script here
DROP INDEX IF EXISTS files_owner_id_parent_id_idx;

DROP TABLE IF EXISTS files;