use serde::{Deserialize, Serialize};

/// 应用状态运行时配置
///
/// 存储应用在运行时需要的敏感配置信息和秘密。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateConfig {
    /// JWT 签名密钥，用于生成和验证令牌
    pub jwt_secret: String,
}
