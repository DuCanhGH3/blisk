use super::auth::UserClaims;
use crate::{
    app::AppState,
    utils::{
        constants::UPLOADS_DIRECTORY,
        errors::AppError,
        futures::flatten,
        response::response,
        structs::{AppJson, AppMultipart},
        uploads::upload_file,
    },
};
use axum::{
    body::{Body, Bytes},
    extract::{Path, State},
    http::{header::CONTENT_TYPE, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use futures::future::TryJoinAll;
use libc::{O_CLOEXEC, O_RDONLY};
use std::io::Read;
use std::path::Path as OsPath;
use tokio::try_join;
use tracing::instrument;
use validator::Validate;

#[derive(TryFromMultipart, Validate)]
pub struct UploadPayload {
    files: Vec<FieldData<Bytes>>,
}
#[derive(serde::Serialize)]
pub struct UploadResponse {
    file_ids: Vec<i64>,
}

#[instrument(name = "Uploading files...", skip(pool, hdfs, claims, files), fields(
    uid = %claims.sub,
    file_name = ?files.iter().map(|file| file.metadata.file_name.clone().unwrap_or("None".to_owned())).collect::<Vec<_>>()
))]
pub async fn upload(
    State(AppState { pool, hdfs, .. }): State<AppState>,
    claims: UserClaims,
    AppMultipart(UploadPayload { files }): AppMultipart<UploadPayload>,
) -> Result<Response, AppError> {
    let mut tasks = Vec::with_capacity(files.len());
    for file in files {
        let pool = pool.clone();
        let hdfs = hdfs.clone();
        tasks.push(flatten(tokio::spawn(async move {
            let mut transaction = pool.begin().await?;
            let file_id = upload_file(&mut transaction, &hdfs, claims.sub, None, file).await?;
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

/// TODO: Proof of concept. Implement additional checks later.
pub async fn load<'c>(
    State(AppState { hdfs, .. }): State<AppState>,
    Path(path): Path<String>,
) -> Result<Response, AppError> {
    let mime = mime_guess::from_path(path.clone())
        .first_raw()
        .map(HeaderValue::from_static)
        .unwrap_or(HeaderValue::from_static("application/octet-stream"));

    let mut buf = Vec::<u8>::new();

    let path = OsPath::new(UPLOADS_DIRECTORY).join(path);

    hdfs.open(path.to_str().unwrap(), O_CLOEXEC | O_RDONLY, None, None)
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    let body = Body::from(buf);

    Ok(([(CONTENT_TYPE, mime)], body).into_response())
}
