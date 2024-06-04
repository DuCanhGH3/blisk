-- Add down migration script here
DROP INDEX IF EXISTS users_id_email_is_verified_idx;

DROP TABLE IF EXISTS users;

DROP TYPE ROLE;
