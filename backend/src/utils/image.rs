use axum::{
    body::Body,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

pub enum Image {
    Filename(String),
    File(String, Vec<u8>),
}

impl IntoResponse for Image {
    fn into_response(self) -> Response {
        match self {
            Self::Filename(name) => (StatusCode::OK, name).into_response(),
            Self::File(filename, data) => {
                let filename_header_value = format!("attachment; filename=\"{filename}\"");

                let mime = mime_guess::from_path(filename)
                    .first_raw()
                    .map(HeaderValue::from_static)
                    .unwrap_or(HeaderValue::from_static("application/octet-stream"));

                Response::builder()
                    .header("Content-Disposition", filename_header_value)
                    .header("Content-Type", mime)
                    .header("Cache-Control", "public, max-age=31536000, immutable")
                    .body(Body::from(data))
                    .unwrap()
            }
        }
    }
}

impl Into<Image> for (String, Vec<u8>) {
    fn into(self) -> Image {
        Image::File(self.0, self.1)
    }
}

impl Into<Image> for String {
    fn into(self) -> Image {
        Image::Filename(self)
    }
}

impl Into<Image> for &str {
    fn into(self) -> Image {
        Image::Filename(self.to_owned())
    }
}
