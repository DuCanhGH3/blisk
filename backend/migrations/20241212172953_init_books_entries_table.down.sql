-- Add down migration script here
DROP INDEX IF EXISTS book_entries_uploader_volume_language_idx;

DROP TABLE IF EXISTS book_entries, book_volumes;