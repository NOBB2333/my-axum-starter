use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::shared::password::hash_password;
use crate::{AppError, error::AuthError};

/// 是否启用演示种子数据的环境变量。
///
/// 种子数据本身固定写在代码里，避免把默认账号、邮箱、密码散落到 `.env`。
const SEED_ENABLED_ENV: &str = "APP_SEED_DEMO";

/// 默认演示用户。
///
/// 仅在 `APP_SEED_DEMO=true` 时创建；重复启动会按邮箱幂等跳过。
const DEMO_USERNAME: &str = "demo";
const DEMO_EMAIL: &str = "demo@example.com";
const DEMO_PASSWORD: &str = "demo-password";

/// 按环境变量开关初始化演示种子数据。
///
/// # 参数
/// * `db` - SeaORM 数据库连接
///
/// # 返回
/// 成功初始化或跳过返回 `Ok(())`，数据库/密码哈希失败返回 `AppError`
pub async fn seed_demo_data_if_enabled(db: &DatabaseConnection) -> Result<(), AppError> {
    if std::env::var(SEED_ENABLED_ENV).as_deref() != Ok("true") {
        return Ok(());
    }

    seed_demo_user(db).await?;
    tracing::info!(
        username = DEMO_USERNAME,
        email = DEMO_EMAIL,
        "Demo seed data is ready"
    );
    Ok(())
}

/// 幂等创建默认演示用户。
///
/// 如果邮箱已存在，认为种子数据已经初始化完成，不再覆盖用户已有内容。
async fn seed_demo_user(db: &DatabaseConnection) -> Result<(), AppError> {
    if user::Entity::find()
        .filter(user::Column::Email.eq(DEMO_EMAIL))
        .one(db)
        .await?
        .is_some()
    {
        return Ok(());
    }

    user::ActiveModel {
        username: Set(DEMO_USERNAME.to_string()),
        email: Set(DEMO_EMAIL.to_string()),
        password_hash: Set(
            hash_password(DEMO_PASSWORD).map_err(|error| AuthError::Internal(error.to_string()))?
        ),
        status: Set(0),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(())
}
