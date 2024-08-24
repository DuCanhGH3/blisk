use crate::{
    hdfs::AppHdfs,
    utils::{constants::UPLOADS_DIRECTORY, errors::AppError, validators::path_is_valid},
};
use axum::body::Bytes;
use axum_typed_multipart::FieldData;
use libc::{O_CLOEXEC, O_CREAT, O_WRONLY};
use sqlx::Postgres;
use std::{ffi::OsStr, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum UploadsError {
    #[error("received an invalid filename: {0}")]
    InvalidName(String),
    #[error("received an IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("this error is not expected")]
    Unexpected,
}

fn validate_file_name<'a>(file_path: impl ToString) -> Result<String, UploadsError> {
    let file_path = file_path.to_string();
    if !path_is_valid(file_path.as_str()) {
        return Err(UploadsError::InvalidName(file_path));
    }
    Ok(file_path)
}

pub async fn upload_file<'c>(
    transaction: &mut sqlx::Transaction<'c, Postgres>,
    hdfs: &AppHdfs,
    user_id: i64,
    parent_id: Option<i64>,
    file: FieldData<Bytes>,
) -> Result<i64, AppError> {
    let file_name = validate_file_name(
        file.metadata
            .file_name
            .ok_or(UploadsError::InvalidName("None".to_owned()))?
            .as_str(),
    )?;
    let ext = Path::new(&file_name)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or(UploadsError::InvalidName(file_name.clone()))?;
    let uid = validate_file_name(user_id)?;

    // TODO: add `extension` column
    let fid = {
        if let Some(pid) = parent_id {
            sqlx::query_scalar!(
                "INSERT INTO files (owner_id, parent_id, ext, path)
                VALUES ($1, $2, $3, (SELECT path || TEXT2LTREE(id::VARCHAR(255)) FROM files WHERE id = $2))
                RETURNING id",
                &user_id,
                &pid,
                &ext,
            )
            .fetch_one(&mut **transaction)
        } else {
            sqlx::query_scalar!(
                "INSERT INTO files (owner_id, ext, path) VALUES ($1, $2, 'Top') RETURNING id",
                &user_id,
                &ext,
            )
            .fetch_one(&mut **transaction)
        }
    }
    .await?;

    let path = Path::new(UPLOADS_DIRECTORY)
        .join(uid.to_string())
        .join(format!("{}.{}", fid, ext));

    async {
        let dir = path.parent().ok_or(UploadsError::Unexpected)?;

        hdfs.mkdir(dir.to_str().ok_or(UploadsError::Unexpected)?)?;

        let mut file_stream = std::io::BufWriter::new(
            hdfs.open(
                path.as_path().to_str().unwrap(),
                O_CLOEXEC | O_WRONLY | O_CREAT,
                None,
                None,
            )
            .map_err(|_| UploadsError::Unexpected)?,
        );

        std::io::copy(&mut file.contents.as_ref(), &mut file_stream)?;

        Ok::<_, UploadsError>(())
    }
    .await?;

    Ok(fid)
}
