//! 认证相关错误

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::response::{ApiError, ApiResponse, Domain, ErrorDetail, Reason};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("用户已存在")]
    UserAlreadyExists,

    #[error("用户不存在")]
    UserNotFound,

    #[error("密码错误")]
    InvalidPassword,

    #[error("用户名或邮箱格式错误")]
    InvalidInput,

    #[error("用户已被停用")]
    UserInactive,

    #[error("无效的访问令牌")]
    InvalidToken,

    #[error("内部错误: {0}")]
    Internal(String),
}

impl AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UserAlreadyExists => StatusCode::CONFLICT,
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::InvalidPassword => StatusCode::UNAUTHORIZED,
            Self::InvalidInput => StatusCode::BAD_REQUEST,
            Self::UserInactive => StatusCode::FORBIDDEN,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> Reason {
        match self {
            Self::UserAlreadyExists => Reason::UserAlreadyExists,
            Self::UserNotFound => Reason::UserNotFound,
            Self::InvalidPassword => Reason::InvalidPassword,
            Self::InvalidInput => Reason::InvalidFormat,
            Self::UserInactive => Reason::UserInactive,
            Self::InvalidToken => Reason::InvalidToken,
            Self::Internal(_) => Reason::InternalError,
        }
    }

    fn to_api_error(&self) -> ApiError {
        ApiError::new(self.status_code(), self.to_string()).with_detail(ErrorDetail::with_message(
            Domain::Auth,
            self.reason(),
            self.to_string(),
        ))
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        ApiResponse::error(self.to_api_error()).into_response()
    }
}
