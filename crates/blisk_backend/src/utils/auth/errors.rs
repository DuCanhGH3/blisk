#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("this user either doesn't exist or has already been verified")]
    AlreadyVerified,
    #[error("this user is not valid")]
    Invalid,
    #[error("this error is not expected")]
    Unexpected,
}
