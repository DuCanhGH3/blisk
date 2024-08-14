use super::auth::UserClaims;
use crate::{
    app::AppState,
    utils::{
        errors::AppError,
        futures::flatten,
        response::response,
        structs::{AppJson, AppMultipart},
        uploads::{upload_file, UploadsError},
    },
};
use axum::{body::Bytes, extract::State, http::StatusCode, response::Response};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use futures::future::TryJoinAll;
use tokio::try_join;
use tracing::instrument;

#[derive(TryFromMultipart)]
pub struct UploadPayload {
    files: Vec<FieldData<Bytes>>,
}
#[derive(serde::Serialize)]
pub struct UploadResponse {
    file_ids: Vec<i64>,
}

#[instrument(name = "Uploading files...", skip(pool, claims, files), fields(
    uid = %claims.sub,
    file_name = ?files.iter().map(|file| file.metadata.file_name.clone().unwrap_or("None".to_owned())).collect::<Vec<_>>()
))]
pub async fn upload(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppMultipart(UploadPayload { files }): AppMultipart<UploadPayload>,
) -> Result<Response, AppError> {
    let mut tasks = Vec::with_capacity(files.len());
    for file in files {
        let pool = pool.clone();
        tasks.push(flatten(tokio::spawn(async move {
            let mut transaction = pool.begin().await?;
            let file_name = file
                .metadata
                .file_name
                .ok_or(UploadsError::InvalidName("None".to_owned()))?;
            let file_id = upload_file(
                &mut transaction,
                file_name.as_str(),
                claims.sub,
                None,
                file.contents,
            )
            .await?;
            transaction.commit().await?;
            Ok::<i64, AppError>(file_id)
        })));
    }
    let (file_ids,) = try_join!(tasks.into_iter().collect::<TryJoinAll<_>>())?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(UploadResponse { file_ids }),
    ))
}
