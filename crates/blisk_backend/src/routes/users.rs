#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("user {0} cannot be found")]
    UserNotFound(String),
    #[error("this error is not expected")]
    Unexpected,
}
