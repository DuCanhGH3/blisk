-- Add down migration script here
DROP TRIGGER IF EXISTS after_link_category_to_book on book_to_category;

DROP TRIGGER IF EXISTS after_link_author_to_book on book_to_author;

DROP TRIGGER IF EXISTS after_update_books ON books;

DROP FUNCTION IF EXISTS after_link_to_book, after_update_books;

DROP INDEX IF EXISTS books_text_search_tsv, fkey_books_book_languages;

DROP TABLE IF EXISTS book_to_author, book_to_category, books, book_authors, book_categories, book_languages;