//! 错误域定义

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 错误域
///
/// 标识错误来源的服务或模块，用于 `errors[].domain` 字段。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum Domain {
    /// 全局/通用错误
    #[default]
    Global,

    /// 认证相关错误（登录、注册、令牌等）
    Auth,

    /// 用户相关错误（用户信息、权限等）
    User,

    /// 数据库相关错误（查询、连接、事务等）
    Database,

    /// 配置相关错误（环境变量、配置文件等）
    Config,

    /// 文件相关错误（上传、下载、存储等）
    File,

    /// Redis 相关错误（缓存、会话等）
    Redis,

    /// 数据验证相关错误（格式、范围、必填等）
    Validation,

    /// 速率限制相关错误（请求频率、配额等）
    RateLimit,
}

impl Domain {
    /// 获取字符串表示
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::Auth => "auth",
            Self::User => "user",
            Self::Database => "database",
            Self::Config => "config",
            Self::File => "file",
            Self::Redis => "redis",
            Self::Validation => "validation",
            Self::RateLimit => "rate_limit",
        }
    }
}

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
