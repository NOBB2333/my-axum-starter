//! 配置相关错误

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::response::{ApiError, ApiResponse, Domain, ErrorDetail, Reason};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("缺少必需的环境变量: {0}")]
    MissingVar(String),

    #[error("环境变量 {var} 的值无效: {value}")]
    InvalidValue { var: String, value: String },

    #[error("配置错误: {0}")]
    Invalid(String),

    #[error("解析错误: {0}")]
    Parse(String),
}

impl ConfigError {
    fn reason(&self) -> Reason {
        match self {
            Self::MissingVar(_) => Reason::MissingEnvVar,
            Self::InvalidValue { .. } => Reason::InvalidConfig,
            Self::Invalid(_) => Reason::InvalidConfig,
            Self::Parse(_) => Reason::InvalidConfig,
        }
    }

    fn to_api_error(&self) -> ApiError {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).with_detail(
            ErrorDetail::with_message(Domain::Config, self.reason(), self.to_string()),
        )
    }
}

impl IntoResponse for ConfigError {
    fn into_response(self) -> Response {
        ApiResponse::error(self.to_api_error()).into_response()
    }
}

impl From<std::env::VarError> for ConfigError {
    fn from(e: std::env::VarError) -> Self {
        Self::MissingVar(e.to_string())
    }
}

impl From<std::num::ParseIntError> for ConfigError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::Parse(e.to_string())
    }
}
