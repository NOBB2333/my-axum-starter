use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use super::ErrorCode;

/// 验证错误类型
#[derive(Debug, Error)]
pub enum ValidationError {
    /// validator 库验证失败
    #[error("{0}")]
    ValidatorError(String),

    /// 自定义验证错误
    #[error("{0}")]
    Custom(String),
}

impl ValidationError {
    /// 从 validator::ValidationErrors 创建验证错误
    pub fn from_validator(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let error_msgs = errors
                    .iter()
                    .filter_map(|e| e.message.as_ref())
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{}: {}",
                    field,
                    if error_msgs.is_empty() {
                        "validation failed".to_string()
                    } else {
                        error_msgs
                    }
                )
            })
            .collect();

        ValidationError::ValidatorError(messages.join("; "))
    }

    /// 创建自定义验证错误
    pub fn custom<S: Into<String>>(msg: S) -> Self {
        ValidationError::Custom(msg.into())
    }
}

impl ErrorCode for ValidationError {
    fn error_code(&self) -> u32 {
        11001
    }

    fn error_message(&self) -> String {
        match self {
            ValidationError::ValidatorError(msg) => format!("验证错误：{}", msg),
            ValidationError::Custom(msg) => format!("验证错误：{}", msg),
        }
    }

    fn http_status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let response = self.to_api_response();
        (self.http_status_code(), Json(response)).into_response()
    }
}
