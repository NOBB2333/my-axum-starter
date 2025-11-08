use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use super::ErrorCode;

/// Redis 错误
#[derive(Debug, Error)]
pub enum RedisError {
    #[error("Failed to create Redis pool: {0}")]
    Pool(#[from] deadpool_redis::CreatePoolError),

    #[error("Redis connection error: {0}")]
    Connection(String),

    #[error("Redis operation error: {0}")]
    Operation(String),
}

impl ErrorCode for RedisError {
    fn error_code(&self) -> u32 {
        match self {
            RedisError::Pool(_) => 10001,
            RedisError::Connection(_) => 10002,
            RedisError::Operation(_) => 10003,
        }
    }

    fn error_message(&self) -> String {
        match self {
            RedisError::Pool(e) => format!("Redis 连接池创建失败：{}", e),
            RedisError::Connection(msg) => format!("Redis 连接错误：{}", msg),
            RedisError::Operation(msg) => format!("Redis 操作错误：{}", msg),
        }
    }

    fn http_status_code(&self) -> StatusCode {
        match self {
            RedisError::Pool(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RedisError::Connection(_) => StatusCode::SERVICE_UNAVAILABLE,
            RedisError::Operation(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for RedisError {
    fn into_response(self) -> Response {
        let status = self.http_status_code();
        let response = self.to_api_response();
        (status, Json(response)).into_response()
    }
}
