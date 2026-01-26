//! 文件上传相关错误

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::response::{ApiError, ApiResponse, Domain, ErrorDetail, Reason};

#[derive(Debug, Error)]
pub enum FileUploadError {
    #[error("文件解析错误: {0}")]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    #[error("文件大小超出限制: {0} 字节")]
    TooLarge(usize),

    #[error("文件类型不允许: {0}")]
    TypeNotAllowed(String),

    #[error("上传失败: {0}")]
    Failed(String),

    #[error("缺少必需字段: {0}")]
    MissingField(String),
}

impl FileUploadError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Multipart(_) => StatusCode::BAD_REQUEST,
            Self::TooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            Self::TypeNotAllowed(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::Failed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MissingField(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn reason(&self) -> Reason {
        match self {
            Self::Multipart(_) => Reason::InvalidFormat,
            Self::TooLarge(_) => Reason::FileTooLarge,
            Self::TypeNotAllowed(_) => Reason::FileTypeNotAllowed,
            Self::Failed(_) => Reason::UploadFailed,
            Self::MissingField(_) => Reason::RequiredFieldMissing,
        }
    }

    fn to_api_error(&self) -> ApiError {
        ApiError::new(self.status_code(), self.to_string()).with_detail(ErrorDetail::with_message(
            Domain::File,
            self.reason(),
            self.to_string(),
        ))
    }
}

impl IntoResponse for FileUploadError {
    fn into_response(self) -> Response {
        ApiResponse::error(self.to_api_error()).into_response()
    }
}
