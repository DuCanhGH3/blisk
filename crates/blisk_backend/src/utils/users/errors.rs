#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("this user either doesn't exist or has already been verified")]
    AlreadyVerified,
    #[error("this user is not valid")]
    Invalid,
    #[error("this error is not expected")]
    Unexpected,
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("user {0} cannot be found")]
    UserNotFound(String),
    #[error("this error is not expected")]
    Unexpected,
}
