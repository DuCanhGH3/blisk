-- Add down migration script here
DROP VIEW IF EXISTS books_view;

DROP FUNCTION IF EXISTS construct_book_reaction_object;
