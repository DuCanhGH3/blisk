#[derive(Debug, thiserror::Error)]
pub enum UploadsError {
    #[error("received an invalid filename: {0}")]
    InvalidName(String),
    #[error("received an IO error: {0}")]
    IoError(#[from] std::io::Error),
}
