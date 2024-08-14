use tokio::task::JoinHandle;

use super::errors::AppError;

pub async fn flatten<T>(handle: JoinHandle<Result<T, AppError>>) -> Result<T, AppError> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(_) => Err(AppError::Unexpected("Handling failed")),
    }
}
