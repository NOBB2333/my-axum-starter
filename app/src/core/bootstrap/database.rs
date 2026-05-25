use crate::{AppError, ConfigError};

/// 确保目标 PostgreSQL 数据库存在。
///
/// 对非 PostgreSQL URL 直接跳过；PostgreSQL URL 会先连接默认 `postgres`
/// 管理库，若目标库不存在则尝试创建。
pub async fn ensure_database_exists(database_url: &str) -> Result<(), AppError> {
    use sea_orm::ConnectionTrait;

    let parsed = url::Url::parse(database_url).map_err(|error| {
        AppError::Config(ConfigError::Invalid(format!(
            "数据库 URL 解析失败: {error}"
        )))
    })?;

    if parsed.scheme() != "postgres" && parsed.scheme() != "postgresql" {
        return Ok(());
    }

    let db_name = parsed.path().trim_start_matches('/');
    if db_name.is_empty() {
        return Ok(());
    }

    let mut admin_url = parsed.clone();
    admin_url.set_path("/postgres");

    let admin_conn = sea_orm::Database::connect(admin_url.as_str())
        .await
        .map_err(|error| {
            AppError::Config(ConfigError::Invalid(format!(
                "无法连接到 PostgreSQL 管理库 '{admin_url}': {error}"
            )))
        })?;

    let exists = admin_conn
        .query_one(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT 1 FROM pg_database WHERE datname = $1",
            [db_name.to_string().into()],
        ))
        .await?
        .is_some();

    if exists {
        tracing::info!("数据库 '{}' 已存在", db_name);
        return Ok(());
    }

    tracing::info!("数据库 '{}' 不存在，正在自动创建", db_name);
    admin_conn
        .execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            format!("CREATE DATABASE {}", quote_pg_identifier(db_name)),
        ))
        .await
        .map_err(|error| {
            AppError::Config(ConfigError::Invalid(database_create_error_message(
                db_name, &error,
            )))
        })?;

    tracing::info!("数据库 '{}' 创建成功", db_name);
    Ok(())
}

fn quote_pg_identifier(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

pub fn database_create_error_message(db_name: &str, error: &impl std::fmt::Display) -> String {
    format!(
        "创建数据库 '{db_name}' 失败: {error}。请确认当前 PostgreSQL 用户拥有 CREATEDB 权限，或提前手动创建数据库后再启动应用。"
    )
}
