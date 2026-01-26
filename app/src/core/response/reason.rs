//! 错误原因定义
//!
//! 机器可读的错误标识符，用于 `errors[].reason` 字段。
//! 使用 UPPER_SNAKE_CASE 格式强调其作为常量标识符的性质。

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 错误原因
///
/// 每个 reason 在特定 domain 内唯一标识一种错误。
/// 客户端可以根据 `(domain, reason)` 组合来编程处理特定错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default)]
pub enum Reason {
    // ==================== 认证 (auth) ====================
    /// 用户不存在
    UserNotFound,

    /// 用户已存在
    UserAlreadyExists,

    /// 密码错误
    InvalidPassword,

    /// 用户已被停用
    UserInactive,

    /// 访问令牌无效
    InvalidToken,

    /// 访问令牌已过期
    TokenExpired,

    /// 缺少认证凭据
    MissingCredentials,

    /// 认证失败
    AuthenticationFailed,

    // ==================== 验证 (validation) ====================
    /// 格式无效
    InvalidFormat,

    /// 缺少必需字段
    RequiredFieldMissing,

    /// 值超出有效范围
    ValueOutOfRange,

    /// 长度无效
    InvalidLength,

    /// 邮箱格式无效
    InvalidEmail,

    /// 用户名格式无效
    InvalidUsername,

    /// 密码强度不足
    WeakPassword,

    /// 两次密码不匹配
    PasswordMismatch,

    // ==================== 资源通用 ====================
    /// 资源未找到
    NotFound,

    /// 资源已存在
    AlreadyExists,

    /// 资源冲突
    Conflict,

    // ==================== 文件 (file) ====================
    /// 文件大小超出限制
    FileTooLarge,

    /// 文件类型不允许
    FileTypeNotAllowed,

    /// 上传失败
    UploadFailed,

    // ==================== 数据库 (database) ====================
    /// 连接失败
    ConnectionFailed,

    /// 查询失败
    QueryFailed,

    /// 事务失败
    TransactionFailed,

    /// 数据重复
    DuplicateEntry,

    // ==================== 配置 (config) ====================
    /// 配置项缺失
    MissingConfig,

    /// 配置值无效
    InvalidConfig,

    /// 环境变量缺失
    MissingEnvVar,

    // ==================== Redis ====================
    /// Redis 连接失败
    RedisConnectionFailed,

    /// Redis 操作失败
    RedisOperationFailed,

    // ==================== 限流 (rate_limit) ====================
    /// 请求频率超限
    RateLimitExceeded,

    /// 配额耗尽
    QuotaExceeded,

    // ==================== 通用 ====================
    /// 内部服务器错误
    InternalError,

    /// 服务暂时不可用
    ServiceUnavailable,

    /// 功能未实现
    NotImplemented,

    /// 请求超时
    Timeout,

    /// 未知错误
    #[default]
    Unknown,
}

impl Reason {
    /// 获取字符串表示
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::UserAlreadyExists => "USER_ALREADY_EXISTS",
            Self::InvalidPassword => "INVALID_PASSWORD",
            Self::UserInactive => "USER_INACTIVE",
            Self::InvalidToken => "INVALID_TOKEN",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::MissingCredentials => "MISSING_CREDENTIALS",
            Self::AuthenticationFailed => "AUTHENTICATION_FAILED",
            Self::InvalidFormat => "INVALID_FORMAT",
            Self::RequiredFieldMissing => "REQUIRED_FIELD_MISSING",
            Self::ValueOutOfRange => "VALUE_OUT_OF_RANGE",
            Self::InvalidLength => "INVALID_LENGTH",
            Self::InvalidEmail => "INVALID_EMAIL",
            Self::InvalidUsername => "INVALID_USERNAME",
            Self::WeakPassword => "WEAK_PASSWORD",
            Self::PasswordMismatch => "PASSWORD_MISMATCH",
            Self::NotFound => "NOT_FOUND",
            Self::AlreadyExists => "ALREADY_EXISTS",
            Self::Conflict => "CONFLICT",
            Self::FileTooLarge => "FILE_TOO_LARGE",
            Self::FileTypeNotAllowed => "FILE_TYPE_NOT_ALLOWED",
            Self::UploadFailed => "UPLOAD_FAILED",
            Self::ConnectionFailed => "CONNECTION_FAILED",
            Self::QueryFailed => "QUERY_FAILED",
            Self::TransactionFailed => "TRANSACTION_FAILED",
            Self::DuplicateEntry => "DUPLICATE_ENTRY",
            Self::MissingConfig => "MISSING_CONFIG",
            Self::InvalidConfig => "INVALID_CONFIG",
            Self::MissingEnvVar => "MISSING_ENV_VAR",
            Self::RedisConnectionFailed => "REDIS_CONNECTION_FAILED",
            Self::RedisOperationFailed => "REDIS_OPERATION_FAILED",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::QuotaExceeded => "QUOTA_EXCEEDED",
            Self::InternalError => "INTERNAL_ERROR",
            Self::ServiceUnavailable => "SERVICE_UNAVAILABLE",
            Self::NotImplemented => "NOT_IMPLEMENTED",
            Self::Timeout => "TIMEOUT",
            Self::Unknown => "UNKNOWN",
        }
    }

    /// 获取默认错误消息
    pub const fn default_message(self) -> &'static str {
        match self {
            Self::UserNotFound => "用户不存在",
            Self::UserAlreadyExists => "用户已存在",
            Self::InvalidPassword => "密码错误",
            Self::UserInactive => "用户已被停用",
            Self::InvalidToken => "无效的访问令牌",
            Self::TokenExpired => "访问令牌已过期",
            Self::MissingCredentials => "缺少认证凭据",
            Self::AuthenticationFailed => "认证失败",
            Self::InvalidFormat => "格式无效",
            Self::RequiredFieldMissing => "缺少必需字段",
            Self::ValueOutOfRange => "值超出有效范围",
            Self::InvalidLength => "长度无效",
            Self::InvalidEmail => "邮箱格式无效",
            Self::InvalidUsername => "用户名格式无效",
            Self::WeakPassword => "密码强度不足",
            Self::PasswordMismatch => "两次输入的密码不一致",
            Self::NotFound => "资源未找到",
            Self::AlreadyExists => "资源已存在",
            Self::Conflict => "资源冲突",
            Self::FileTooLarge => "文件大小超出限制",
            Self::FileTypeNotAllowed => "文件类型不允许",
            Self::UploadFailed => "上传失败",
            Self::ConnectionFailed => "连接失败",
            Self::QueryFailed => "查询失败",
            Self::TransactionFailed => "事务失败",
            Self::DuplicateEntry => "数据已存在",
            Self::MissingConfig => "配置项缺失",
            Self::InvalidConfig => "配置值无效",
            Self::MissingEnvVar => "环境变量缺失",
            Self::RedisConnectionFailed => "Redis 连接失败",
            Self::RedisOperationFailed => "Redis 操作失败",
            Self::RateLimitExceeded => "请求过于频繁，请稍后重试",
            Self::QuotaExceeded => "配额已耗尽",
            Self::InternalError => "内部服务器错误",
            Self::ServiceUnavailable => "服务暂时不可用",
            Self::NotImplemented => "功能未实现",
            Self::Timeout => "请求超时",
            Self::Unknown => "未知错误",
        }
    }
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
