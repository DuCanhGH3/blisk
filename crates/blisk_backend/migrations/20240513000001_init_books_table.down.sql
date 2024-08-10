-- Add down migration script here
DROP INDEX IF EXISTS fkey_books_book_languages;

DROP TABLE IF EXISTS book_to_author, book_to_category, books, book_authors, book_categories, book_languages;