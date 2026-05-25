/// 核心功能模块（配置、日志、中间件等）
pub mod core;
/// 错误处理模块
pub mod error;
/// 业务功能模块
pub mod modules;
/// API路由模块
pub mod routes;
/// 共享工具模块（JWT、密码等）
pub mod shared;

pub use core::*;
pub use error::*;
pub use modules::*;
pub use routes::v1;
