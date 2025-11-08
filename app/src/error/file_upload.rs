use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use super::ErrorCode;

#[derive(Debug, Error)]
pub enum FileUploadError {
    #[error("Multipart error: {0}")]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    #[error("File size exceeds the limit: {0} bytes")]
    FileSizeExceeded(usize),

    #[error("File type not allowed: {0}")]
    FileTypeNotAllowed(String),

    #[error("File upload failed: {0}")]
    UploadFailed(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

impl ErrorCode for FileUploadError {
    fn error_code(&self) -> u32 {
        match self {
            FileUploadError::Multipart(_) => 11101,
            FileUploadError::FileSizeExceeded(_) => 11102,
            FileUploadError::FileTypeNotAllowed(_) => 11103,
            FileUploadError::UploadFailed(_) => 11104,
            FileUploadError::MissingField(_) => 11105,
        }
    }

    fn error_message(&self) -> String {
        match self {
            FileUploadError::Multipart(e) => format!("文件上传表单解析错误：{}", e),
            FileUploadError::FileSizeExceeded(size) => {
                format!("文件大小超出限制：{} 字节", size)
            }
            FileUploadError::FileTypeNotAllowed(t) => format!("文件类型不允许：{}", t),
            FileUploadError::UploadFailed(msg) => format!("文件上传失败：{}", msg),
            FileUploadError::MissingField(field) => format!("缺少必填字段：{}", field),
        }
    }

    fn http_status_code(&self) -> StatusCode {
        match self {
            FileUploadError::Multipart(_) => StatusCode::BAD_REQUEST,
            FileUploadError::FileSizeExceeded(_) => StatusCode::PAYLOAD_TOO_LARGE,
            FileUploadError::FileTypeNotAllowed(_) => StatusCode::BAD_REQUEST,
            FileUploadError::UploadFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            FileUploadError::MissingField(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for FileUploadError {
    fn into_response(self) -> Response {
        let status = self.http_status_code();
        let response = self.to_api_response();
        (status, Json(response)).into_response()
    }
}
