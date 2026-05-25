//! 应用核心功能模块
//!
//! 包含配置、日志、中间件、应用状态等核心功能。

mod bootstrap;
pub mod config;
mod cors;
mod http;
mod logging;
pub mod middleware;
mod rate_limit;
pub mod response;
pub mod state;

/// 数据库启动前置检查
pub use bootstrap::{
    database_create_error_message, ensure_database_exists, seed_demo_data_if_enabled,
};
/// 应用全局配置
pub use config::AppConfig;
/// CORS 跨域配置构建函数
pub use cors::build_cors_layer;
/// 分页请求解析和约束
pub use http::{
    DEFAULT_PAGE, DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE, MIN_PAGE_SIZE, Pagination, PaginationQuery,
};
/// 旧日志文件清理函数
pub use logging::cleanup_old_logs;
/// 速率限制错误处理函数
pub use rate_limit::handle_rate_limit_error;
/// 标准 API 响应格式
pub use response::{API_VERSION, ApiResponse, Domain, ErrorDetail};
/// 应用状态（包含数据库、Redis等）
pub use state::AppState;
