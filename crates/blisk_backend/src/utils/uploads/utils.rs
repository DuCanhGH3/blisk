use super::errors::UploadsError;
use crate::utils::{constants::UPLOADS_DIRECTORY, errors::AppError, validators::path_is_valid};
use axum::body::Bytes;
use sqlx::Postgres;
use std::{ffi::OsStr, path::Path};

fn validate_file_name<'a>(file_path: impl ToString) -> Result<String, UploadsError> {
    let file_path = file_path.to_string();
    if !path_is_valid(file_path.as_str()) {
        return Err(UploadsError::InvalidName(file_path));
    }
    Ok(file_path)
}

pub async fn upload_file<'c>(
    transaction: &mut sqlx::Transaction<'c, Postgres>,
    file_name: &str,
    user_id: i64,
    parent_id: Option<i64>,
    bytes: Bytes,
) -> Result<(), AppError> {
    let file_name = validate_file_name(file_name)?;
    let ext = Path::new(&file_name)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or(UploadsError::InvalidName(file_name.clone()))?;
    let uid = validate_file_name(user_id)?;

    let fid = {
        if let Some(pid) = parent_id {
            sqlx::query_scalar!(
                "INSERT INTO files (owner_id, parent_id, path)
                VALUES ($1, $2, (SELECT path || TEXT2LTREE(id::VARCHAR(255)) FROM files WHERE id = $2))
                RETURNING id",
                &user_id,
                &pid
            )
            .fetch_one(&mut **transaction)
        } else {
            sqlx::query_scalar!(
                "INSERT INTO files (owner_id, path) VALUES ($1, 'Top') RETURNING id",
                &user_id
            )
            .fetch_one(&mut **transaction)
        }
    }
    .await?;

    let path = Path::new(UPLOADS_DIRECTORY)
        .join(uid.to_string())
        .join(format!("{}.{}", fid, ext));

    async {
        let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(path).await?);

        tokio::io::copy(&mut bytes.as_ref(), &mut file).await?;

        Ok::<_, UploadsError>(())
    }
    .await
    .map_err(|err| AppError::from(err))?;

    Ok(())
}
