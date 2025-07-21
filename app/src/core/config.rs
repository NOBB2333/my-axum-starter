use crate::error::{AppError, EnvConfigError};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

/// 应用程序主配置结构
/// 
/// 包含服务器、数据库、日志和安全相关的配置项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)] 
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub secrets: SecretsConfig,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    /// 服务器绑定地址
    pub host: String,
    /// 服务器端口
    pub port: u16,
    /// 请求超时时间（秒）
    pub timeout: u64,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    /// 数据库连接URL
    pub url: String,
    /// 连接池最大连接数
    pub max_connections: u32,
    /// 连接池超时时间（秒）
    pub pool_timeout: u64,
}

/// 增强的日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    /// 日志级别 (trace, debug, info, warn, error)
    pub level: String,
    /// 控制台日志格式 (pretty, compact) - 文件始终用JSON
    pub console_format: String,
    /// 是否输出到控制台
    pub console: bool,
    /// 是否输出到文件
    pub file: bool,
    /// 日志文件目录
    pub file_dir: String,
    /// 日志文件名前缀
    pub file_prefix: String,
    /// 日志轮转方式 (daily, hourly, never)
    pub rotation: String,
    /// 保留的日志文件数量（0表示不限制）
    pub max_files: usize,
}

impl LoggingConfig {
    /// 获取带环境后缀的文件前缀
    pub fn get_file_prefix_with_env(&self) -> String {
        let env_suffix = match self.level.as_str() {
            "trace" | "debug" => "-dev",
            _ => "-prod",
        };
        format!("{}{}", self.file_prefix, env_suffix)
    }
}

/// 敏感信息配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SecretsConfig {
    /// JWT 签名密钥
    pub jwt_secret: String,
    /// Redis 连接URL（可选）
    pub redis_url: Option<String>,
}

impl AppConfig {
    /// 从配置文件和环境变量加载配置
    pub fn load() -> Result<Self, EnvConfigError> {
        // 加载 .env 文件，如果文件不存在则忽略
        dotenvy::dotenv().ok();

        // 构建配置层次结构
        let figment = Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::prefixed("APP_"));

        let mut config: AppConfig = figment.extract()?;

        // 设置敏感环境变量
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            config.database.url = database_url;
        }

        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            config.secrets.jwt_secret = jwt_secret;
        }

        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            config.secrets.redis_url = Some(redis_url);
        }

        // 生产环境日志配置覆盖
        if let Ok(log_file_dir) = std::env::var("LOG_FILE_DIR") {
            config.logging.file_dir = log_file_dir;
            config.logging.file = true; // 自动启用文件日志
        }

        if let Ok(log_rotation) = std::env::var("LOG_ROTATION") {
            config.logging.rotation = log_rotation;
        }

        if let Ok(log_console) = std::env::var("LOG_CONSOLE") {
            config.logging.console = log_console.parse().unwrap_or(true);
        }

        // 验证必需的配置项
        if config.database.url.is_empty() {
            return Err(EnvConfigError::MissingVar {
                var_name: "DATABASE_URL".to_string(),
            });
        }

        if config.secrets.jwt_secret.is_empty() {
            return Err(EnvConfigError::MissingVar {
                var_name: "JWT_SECRET".to_string(),
            });
        }

        Ok(config)
    }

    /// 获取服务器监听地址
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// 初始化日志系统
    pub fn init_tracing(&self) -> Result<(), AppError> {
        crate::core::logging::init_tracing(&self.logging)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            logging: LoggingConfig::default(),
            secrets: SecretsConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            timeout: 30,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            max_connections: 10,
            pool_timeout: 30,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            console_format: "pretty".to_string(), 
            console: true,
            file: false,
            file_dir: "./logs".to_string(),
            file_prefix: "app".to_string(),
            rotation: "daily".to_string(),
            max_files: 30, // 默认保留30天
        }
    }
}

impl Default for SecretsConfig {
    fn default() -> Self {
        Self {
            jwt_secret: String::new(),
            redis_url: None,
        }
    }
}