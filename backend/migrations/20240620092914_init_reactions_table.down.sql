-- Add down migration script here
DROP TRIGGER IF EXISTS recalculate_comment_reactions ON comment_reactions;

DROP TRIGGER IF EXISTS recalculate_posts_reactions ON post_reactions;

DROP FUNCTION IF EXISTS recalculate_comment_reactions, recalculate_posts_reactions, calculate_reaction_delta;

DROP TABLE IF EXISTS comment_reactions_tally, posts_reactions_tally, comment_reactions, post_reactions;

DROP TYPE PREACT;