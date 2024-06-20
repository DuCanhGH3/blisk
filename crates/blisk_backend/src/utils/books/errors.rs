#[derive(Debug, thiserror::Error)]
pub enum BooksError {
    #[error("book {0} cannot be found")]
    BookNotFound(i64),
}
