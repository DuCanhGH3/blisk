-- Add down migration script here
DROP TABLE IF EXISTS comment_reactions, post_reactions;

DROP TYPE PREACT;