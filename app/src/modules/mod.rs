//! 业务功能模块
//!
//! 包含应用的各项业务功能实现，如用户管理等。

/// API 文档路由
mod docs;
/// 404 处理
mod not_found;
/// 用户管理模块（注册、登录、获取用户信息）
pub mod user;

pub use docs::*;
pub use not_found::*;
