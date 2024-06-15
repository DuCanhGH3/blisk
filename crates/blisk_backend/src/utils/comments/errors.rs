#[derive(Debug, thiserror::Error)]
pub enum CommentsError {
    #[error("comment {0} cannot be found")]
    CommentNotFound(i64),
    #[error("this error is not expected")]
    Unexpected,
}
