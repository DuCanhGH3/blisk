-- Add down migration script here
DROP FUNCTION IF EXISTS create_comment_reaction, create_post_reaction, fetch_comments, fetch_posts, fetch_replies, construct_reaction_object;
