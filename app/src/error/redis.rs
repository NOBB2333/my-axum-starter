//! Redis 相关错误

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::response::{ApiError, ApiResponse, Domain, ErrorDetail, Reason};

#[derive(Debug, Error)]
pub enum RedisError {
    #[error("Redis 连接池创建失败: {0}")]
    Pool(#[from] deadpool_redis::CreatePoolError),

    #[error("Redis 连接错误: {0}")]
    Connection(String),

    #[error("Redis 操作错误: {0}")]
    Operation(String),
}

impl RedisError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Pool(_) | Self::Connection(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::Operation(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> Reason {
        match self {
            Self::Pool(_) | Self::Connection(_) => Reason::RedisConnectionFailed,
            Self::Operation(_) => Reason::RedisOperationFailed,
        }
    }

    fn to_api_error(&self) -> ApiError {
        ApiError::new(self.status_code(), self.to_string()).with_detail(ErrorDetail::with_message(
            Domain::Redis,
            self.reason(),
            self.to_string(),
        ))
    }
}

impl IntoResponse for RedisError {
    fn into_response(self) -> Response {
        ApiResponse::error(self.to_api_error()).into_response()
    }
}
