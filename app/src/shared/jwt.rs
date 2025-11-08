use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 ID
    pub sub: i32,

    /// 过期时间（Unix timestamp）
    pub exp: i64,

    /// 签发时间（Unix timestamp）
    pub iat: i64,
}

impl Claims {
    /// 创建新的 claims，默认过期时间为 7 天
    pub fn new(user_id: i32, expires_in_secs: i64) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub: user_id,
            exp: now + expires_in_secs,
            iat: now,
        }
    }
}

/// JWT 服务
#[derive(Clone, Debug)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    /// 创建新的 JWT 服务
    pub fn new(secret: String) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    /// 生成 JWT token
    ///
    /// # 参数
    /// * `user_id` - 用户 ID
    /// * `expires_in_secs` - 过期时间（秒）
    ///
    /// # 返回
    /// 返回生成的 token 字符串
    pub fn generate_token(
        &self,
        user_id: i32,
        expires_in_secs: i64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, expires_in_secs);
        encode(&Header::default(), &claims, &self.encoding_key)
    }

    /// 验证并解析 JWT token
    ///
    /// # 参数
    /// * `token` - JWT token 字符串
    ///
    /// # 返回
    /// 返回解析后的 token 数据，包含 claims 和 header
    pub fn verify_token(
        &self,
        token: &str,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
    }

    /// 从 token 中提取用户 ID
    ///
    /// # 参数
    /// * `token` - JWT token 字符串
    ///
    /// # 返回
    /// 返回用户 ID，如果 token 无效或过期则返回错误
    pub fn extract_user_id(&self, token: &str) -> Result<i32, jsonwebtoken::errors::Error> {
        self.verify_token(token).map(|data| data.claims.sub)
    }
}
