#[derive(Debug, thiserror::Error)]
pub enum PostsError {
    #[error("post {0} cannot be found")]
    PostNotFound(i64),
    #[error("this error is not expected")]
    Unexpected,
}
