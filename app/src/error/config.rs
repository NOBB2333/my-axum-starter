use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use super::ErrorCode;

#[derive(Debug, Error)]
pub enum EnvConfigError {
    #[error("Missing required environment variable: {var_name}")]
    MissingVar { var_name: String },

    #[error("Invalid value for {var_name}: {value}")]
    InvalidValue { var_name: String, value: String },

    #[error("Configuration error: {0}")]
    InvalidConfig(String),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("Parse error for {var_name}: {source}")]
    ParseError {
        var_name: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl From<std::num::ParseIntError> for EnvConfigError {
    fn from(err: std::num::ParseIntError) -> Self {
        EnvConfigError::ParseError {
            var_name: "unknown".to_string(),
            source: Box::new(err),
        }
    }
}

impl ErrorCode for EnvConfigError {
    fn error_code(&self) -> u32 {
        match self {
            EnvConfigError::MissingVar { .. } => 10201,
            EnvConfigError::InvalidValue { .. } => 10202,
            EnvConfigError::InvalidConfig(_) => 10203,
            EnvConfigError::EnvVar(_) => 10204,
            EnvConfigError::ParseError { .. } => 10205,
        }
    }

    fn error_message(&self) -> String {
        match self {
            EnvConfigError::MissingVar { var_name } => format!("缺少必需的环境变量：{}", var_name),
            EnvConfigError::InvalidValue { var_name, value } => {
                format!("环境变量 {} 的值无效：{}", var_name, value)
            }
            EnvConfigError::InvalidConfig(msg) => format!("配置错误：{}", msg),
            EnvConfigError::EnvVar(e) => format!("环境变量错误：{}", e),
            EnvConfigError::ParseError { var_name, source } => {
                format!("环境变量 {} 解析错误：{}", var_name, source)
            }
        }
    }

    fn http_status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl IntoResponse for EnvConfigError {
    fn into_response(self) -> Response {
        let status = self.http_status_code();
        let response = self.to_api_response();
        (status, Json(response)).into_response()
    }
}
