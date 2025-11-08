mod auth;
mod config;
mod file_upload;
mod redis;
mod validation;

use aide::OperationOutput;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use thiserror::Error;

use crate::ApiResponse;
pub use auth::AuthError;
pub use config::*;
pub use file_upload::FileUploadError;
pub use redis::RedisError;
pub use validation::ValidationError;

/// 错误码 Trait
///
/// 为各个错误类型定义业务错误码、错误消息和 HTTP 状态码的映射。
/// 通过实现此 trait，每个错误类型可以独立定义自己的错误响应格式。
pub trait ErrorCode: Error {
    /// 获取业务错误码（用于 ApiResponse 的 code 字段）
    fn error_code(&self) -> u32;

    /// 获取错误消息（用于 ApiResponse 的 msg 字段）
    fn error_message(&self) -> String {
        self.to_string()
    }

    /// 获取 HTTP 状态码
    fn http_status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    /// 构建 ApiResponse 错误响应
    fn to_api_response(&self) -> ApiResponse<()> {
        ApiResponse::new(self.error_code(), self.error_message(), None)
    }
}

/// 应用程序错误枚举
///
/// 统一处理应用程序中可能出现的各种错误类型
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] EnvConfigError),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("HTTP request error: {status}")]
    Http { status: StatusCode },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("General error: {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("File handle error: {0}")]
    FileHandle(#[from] FileUploadError),

    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            // 直接委托给具体的错误类型处理
            AppError::Redis(err) => err.into_response(),
            AppError::FileHandle(err) => err.into_response(),
            AppError::Config(err) => err.into_response(),
            AppError::Validation(err) => err.into_response(),
            AppError::Auth(err) => err.into_response(),

            // 其他错误类型的通用处理
            AppError::Database(_) => {
                let response = ApiResponse::<()>::new(10101, "数据库错误".to_string(), None);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            }
            AppError::Io(_) => {
                let response = ApiResponse::<()>::new(10301, "IO 错误".to_string(), None);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            }
            AppError::Serde(err) => {
                let response =
                    ApiResponse::<()>::new(10302, format!("数据格式错误：{}", err), None);
                (StatusCode::BAD_REQUEST, Json(response)).into_response()
            }
            AppError::Anyhow(err) => {
                let response = ApiResponse::<()>::new(10303, format!("内部错误：{}", err), None);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            }
            AppError::Http { status } => {
                let response = ApiResponse::<()>::new(
                    status.as_u16() as u32,
                    format!("HTTP 错误：{}", status),
                    None,
                );
                (status, Json(response)).into_response()
            }
        }
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(err: axum::extract::multipart::MultipartError) -> Self {
        AppError::FileHandle(FileUploadError::Multipart(err))
    }
}

impl From<deadpool_redis::CreatePoolError> for AppError {
    fn from(err: deadpool_redis::CreatePoolError) -> Self {
        AppError::Redis(RedisError::Pool(err))
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::Validation(ValidationError::from_validator(err))
    }
}

impl OperationOutput for AppError {
    type Inner = ();
}
