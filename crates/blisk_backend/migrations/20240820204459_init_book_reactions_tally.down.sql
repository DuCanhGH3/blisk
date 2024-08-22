-- Add down migration script here
DROP FUNCTION IF EXISTS books_boost_rating;

DROP TRIGGER IF EXISTS recalculate_books_reactions ON posts;

DROP FUNCTION IF EXISTS recalculate_books_reactions, calculate_book_reaction_delta;

DROP TABLE IF EXISTS book_reactions_tally;
