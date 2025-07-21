use crate::{core::config::LoggingConfig, error::AppError};
use std::fs;
use tracing_appender::non_blocking;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing(config: &LoggingConfig) -> Result<(), AppError> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));

    if config.file {
        fs::create_dir_all(&config.file_dir).map_err(|e| {
            AppError::Io(e)
        })?;
    }

    match (config.console, config.file) {
        (true, true) => {
            // 同时输出到控制台和文件
            let (file_writer, _file_guard) = create_file_appender(config)?;
            let (console_writer, _console_guard) = non_blocking(std::io::stdout());

            let registry = tracing_subscriber::registry().with(env_filter);

            match config.console_format.as_str() {
                "pretty" => {
                    registry
                        .with(
                            tracing_subscriber::fmt::layer()
                                .pretty() // 控制台用 pretty->易读
                                .with_writer(console_writer)
                                .with_ansi(true)
                                .with_file(true)
                                .with_line_number(true)
                                .with_target(false)
                        )
                        .with(
                            tracing_subscriber::fmt::layer()
                                .json() // 文件永远用 JSON
                                .with_writer(file_writer)
                                .with_file(true)
                                .with_line_number(true)
                                .with_target(false)
                                .with_ansi(false)
                        )
                        .init();
                }
                _ => {
                    // 默认使用 compact 格式
                    registry
                        .with(
                            tracing_subscriber::fmt::layer()
                                .compact()
                                .with_writer(console_writer)
                                .with_ansi(true) 
                                .with_file(true)
                                .with_line_number(true)
                                .with_target(false)
                        )
                        .with(
                            tracing_subscriber::fmt::layer()
                                .json()
                                .with_writer(file_writer)
                                .with_file(true)
                                .with_line_number(true)
                                .with_target(false)
                                .with_ansi(false)
                        )
                        .init();
                }
            }

            std::mem::forget(_file_guard);
            std::mem::forget(_console_guard);
        }
        (true, false) => {
            // 仅输出到控制台
            let (console_writer, _console_guard) = non_blocking(std::io::stdout());

            let registry = tracing_subscriber::registry()
                .with(env_filter);

            match config.console_format.as_str() {
                "pretty" => {
                    registry
                        .with(
                            tracing_subscriber::fmt::layer()
                                .pretty()
                                .with_writer(console_writer)
                                .with_ansi(true)
                                .with_file(true)
                                .with_line_number(true)
                                .with_target(false)
                        )
                        .init();
                }
                _ => {
                    registry
                        .with(
                            tracing_subscriber::fmt::layer()
                                .compact()
                                .with_writer(console_writer)
                                .with_ansi(true)
                                .with_file(true)
                                .with_line_number(true)
                                .with_target(false)
                        )
                        .init();
                }
            }

            std::mem::forget(_console_guard);
        }
        (false, true) => {
            // 仅输出到文件
            let (file_writer, _file_guard) = create_file_appender(config)?;

            let registry = tracing_subscriber::registry()
                .with(env_filter);

            registry
                .with(
                    tracing_subscriber::fmt::layer()
                        .json()
                        .with_writer(file_writer)
                        .with_file(true)
                        .with_line_number(true)
                        .with_target(false)
                        .with_ansi(false)
                )
                .init();

            std::mem::forget(_file_guard);
        }
        (false, false) => {
            return Err(AppError::Validation(
                "至少需要启用控制台或文件日志输出".to_string(),
            ));
        }
    }

    tracing::info!(
        "日志系统初始化完成 - 级别: {}, 控制台格式: {}, 控制台: {}, 文件: {}",
        config.level,
        config.console_format,
        config.console,
        config.file
    );

    if config.file {
        tracing::info!("日志文件目录: {}", config.file_dir);
        tracing::info!("日志文件前缀: {}", config.get_file_prefix_with_env());
        tracing::info!("日志轮转策略: {}", config.rotation);
        tracing::info!("保留文件数量: {}", config.max_files);
    }

    Ok(())
}

fn create_file_appender(
    config: &LoggingConfig,
) -> Result<(non_blocking::NonBlocking, non_blocking::WorkerGuard), AppError> {
    use tracing_appender::rolling::{RollingFileAppender, Rotation};
    
    let file_prefix_with_env = config.get_file_prefix_with_env();
    
    let rotation = match config.rotation.as_str() {
        "daily" => Rotation::DAILY,
        "hourly" => Rotation::HOURLY,
        "never" => Rotation::NEVER,
        _ => {
            return Err(AppError::Validation(format!(
                "不支持的日志轮转策略: {}，支持的策略: daily, hourly, never",
                config.rotation
            )));
        }
    };

    let file_appender = RollingFileAppender::builder()
        .rotation(rotation)
        .filename_prefix(&file_prefix_with_env)
        .filename_suffix("log")
        .build(&config.file_dir)
        .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    Ok(non_blocking(file_appender))
}

// 日志文件清理功能
pub fn cleanup_old_logs(config: &LoggingConfig) -> Result<(), AppError> {
    if config.max_files == 0 {
        return Ok(()); // 0 表示不限制文件数量
    }

    let log_dir = std::path::Path::new(&config.file_dir);
    if !log_dir.exists() {
        return Ok(());
    }

    let file_prefix_with_env = config.get_file_prefix_with_env();

    let mut log_files: Vec<_> = fs::read_dir(log_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name()?.to_str()?;
                // 匹配带环境后缀的日志文件
                if file_name.starts_with(&file_prefix_with_env) && file_name.ends_with(".log") {
                    let metadata = entry.metadata().ok()?;
                    let modified = metadata.modified().ok()?;
                    return Some((path, modified));
                }
            }
            None
        })
        .collect();

    log_files.sort_by(|a, b| b.1.cmp(&a.1));

    if log_files.len() > config.max_files {
        for (path, _) in log_files.iter().skip(config.max_files) {
            if let Err(e) = fs::remove_file(path) {
                tracing::warn!("删除旧日志文件失败 {}: {}", path.display(), e);
            } else {
                tracing::info!("已删除旧日志文件: {}", path.display());
            }
        }
    }

    Ok(())
}