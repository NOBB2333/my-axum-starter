use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set, SqlErr, TransactionTrait,
};
use tracing::instrument;

use crate::{
    AppState, Pagination,
    error::AuthError,
    shared::{FromState, jwt::JwtService, password},
};
use entity::user;

use super::dto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UserListItem};

/// 用户服务
///
/// 处理用户注册、登录等业务逻辑
pub struct UserService {
    db: DatabaseConnection,
    jwt_service: JwtService,
}

impl FromState for UserService {
    fn from_state(app: &AppState) -> Self {
        Self {
            db: app.db.clone(),
            jwt_service: app.jwt_service.clone(),
        }
    }
}

impl UserService {
    /// 分页查询用户列表。
    #[instrument(skip(self))]
    pub async fn list_users(
        &self,
        pagination: Pagination,
    ) -> Result<(Vec<UserListItem>, u64), AuthError> {
        let paginator = user::Entity::find()
            .order_by_desc(user::Column::CreatedAt)
            .paginate(&self.db, pagination.page_size);
        let total = paginator
            .num_items()
            .await
            .map_err(|_| AuthError::Internal("数据库查询失败".to_string()))?;
        let rows = paginator
            .fetch_page(pagination.zero_based_page())
            .await
            .map_err(|_| AuthError::Internal("数据库查询失败".to_string()))?;

        let items = rows.into_iter().map(UserListItem::from).collect();
        Ok((items, total))
    }

    /// 用户注册业务逻辑
    ///
    /// 执行以下步骤：
    /// 1. 验证用户名长度（3-20字符）和密码长度（至少8字符）
    /// 2. 验证两次密码输入是否一致
    /// 3. 检查用户名和邮箱是否已存在
    /// 4. 使用Argon2算法哈希密码
    /// 5. 创建新用户并保存到数据库
    ///
    /// # 参数
    /// * `req` - 注册请求，包含用户名、邮箱、密码
    ///
    /// # 返回
    /// 成功返回 RegisterResponse（用户ID、用户名、邮箱）
    /// 失败返回 AuthError（如果用户已存在、验证失败等）
    #[instrument(skip(self, req))]
    pub async fn register(&self, req: RegisterRequest) -> Result<RegisterResponse, AuthError> {
        // 验证用户名
        if req.username.is_empty() || req.username.len() < 3 || req.username.len() > 20 {
            return Err(AuthError::InvalidUsername);
        }

        // 验证密码长度
        if req.password.len() < 8 {
            return Err(AuthError::PasswordTooShort);
        }

        // 验证两次密码是否一致
        if req.password != req.password_confirm {
            return Err(AuthError::PasswordMismatch);
        }

        let txn = self
            .db
            .begin()
            .await
            .map_err(|_| AuthError::Internal("数据库事务启动失败".to_string()))?;

        // 检查用户名是否已存在
        let existing_user = user::Entity::find()
            .filter(user::Column::Username.eq(&req.username))
            .one(&txn)
            .await
            .map_err(|_| AuthError::Internal("数据库查询失败".to_string()))?;

        if existing_user.is_some() {
            return Err(AuthError::UserAlreadyExists);
        }

        // 检查邮箱是否已存在
        let existing_email = user::Entity::find()
            .filter(user::Column::Email.eq(&req.email))
            .one(&txn)
            .await
            .map_err(|_| AuthError::Internal("数据库查询失败".to_string()))?;

        if existing_email.is_some() {
            return Err(AuthError::UserAlreadyExists);
        }

        // 密码加密
        let password_hash = password::hash_password(&req.password)
            .map_err(|e| AuthError::Internal(e.to_string()))?;

        // 保存到数据库
        let new_user = user::ActiveModel {
            username: Set(req.username.clone()),
            email: Set(req.email.clone()),
            password_hash: Set(password_hash),
            status: Set(0), // 激活状态
            ..Default::default()
        };

        let user_model = new_user.insert(&txn).await.map_err(map_insert_user_error)?;

        txn.commit()
            .await
            .map_err(|_| AuthError::Internal("数据库事务提交失败".to_string()))?;

        Ok(RegisterResponse {
            id: user_model.id,
            username: user_model.username,
            email: user_model.email,
        })
    }

    /// 用户登录业务逻辑
    ///
    /// 执行以下步骤：
    /// 1. 根据用户名或邮箱查询用户
    /// 2. 检查用户状态（必须是激活状态）
    /// 3. 验证密码是否正确
    /// 4. 生成有效期为7天的JWT令牌
    ///
    /// # 参数
    /// * `req` - 登录请求，包含用户名/邮箱和密码
    ///
    /// # 返回
    /// 成功返回 LoginResponse（用户信息和JWT令牌）
    /// 失败返回 AuthError（如果用户不存在、密码错误、用户被停用等）
    #[instrument(skip(self, req))]
    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AuthError> {
        // 根据用户名或邮箱查询用户
        let user_model = user::Entity::find()
            .filter(
                Condition::any()
                    .add(user::Column::Username.eq(&req.username_or_email))
                    .add(user::Column::Email.eq(&req.username_or_email)),
            )
            .one(&self.db)
            .await
            .map_err(|_| AuthError::Internal("数据库查询失败".to_string()))?
            .ok_or(AuthError::UserNotFound)?;

        // 检查用户状态
        if user_model.status != 0 {
            return Err(AuthError::UserInactive);
        }

        // 验证密码
        let password_valid = password::verify_password(&req.password, &user_model.password_hash)
            .map_err(|e| AuthError::Internal(e.to_string()))?;

        if !password_valid {
            return Err(AuthError::InvalidPassword);
        }

        // 生成 JWT token
        let token = self
            .jwt_service
            .generate_token(user_model.id, 7 * 24 * 3600) // 7天过期
            .map_err(|e| AuthError::Internal(e.to_string()))?;

        Ok(LoginResponse {
            id: user_model.id,
            username: user_model.username,
            email: user_model.email,
            token,
            expires_in: 7 * 24 * 3600,
        })
    }

    /// 根据用户ID获取用户信息
    ///
    /// 从数据库中查询指定ID的用户信息。
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    ///
    /// # 返回
    /// 成功返回 RegisterResponse（用户ID、用户名、邮箱）
    /// 如果用户不存在返回 AuthError::UserNotFound
    #[instrument(skip(self))]
    pub async fn get_user(&self, user_id: i32) -> Result<RegisterResponse, AuthError> {
        let user_model = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|_| AuthError::Internal("数据库查询失败".to_string()))?
            .ok_or(AuthError::UserNotFound)?;

        Ok(RegisterResponse {
            id: user_model.id,
            username: user_model.username,
            email: user_model.email,
        })
    }
}

fn map_insert_user_error(error: sea_orm::DbErr) -> AuthError {
    if matches!(error.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) {
        return AuthError::UserAlreadyExists;
    }

    tracing::error!(error = %error, "create user failed");
    AuthError::Internal("创建用户失败".to_string())
}

impl From<user::Model> for UserListItem {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            status: model.status,
        }
    }
}
