# My Axum Starter

我的个人 Axum Web 应用模板，用于快速启动新项目。如果对你也有帮助的话就太好了。

## 特性

- **Web框架**: Axum + SeaORM + PostgreSQL
- **认证系统**: JWT令牌认证 + Argon2密码哈希
- **限流控制**: tower_governor IP限流（全局10req/s，认证端2req/s）
- **配置管理**: 支持config.toml和环境变量，支持多种配置格式
- **错误处理**: 统一的错误处理，HTTP状态码+错误原因映射
- **API响应**: 遵循 Google JSON Style Guide 标准，支持分页、资源元数据
- **文档生成**: OpenAPI/Swagger文档（debug模式）
- **中间件栈**: 日志追踪、CORS、压缩、请求ID等
- **日志系统**: 支持文件日志轮转和自动清理

## 项目结构

```shell
├── app/                          # 主应用程序
│   ├── src/
│   │   ├── main.rs
│   │   ├── core/                 # 核心功能
│   │   │   ├── config/           # 配置管理（模块化）
│   │   │   │   ├── mod.rs
│   │   │   │   ├── logging.rs    # 日志配置
│   │   │   │   ├── database.rs   # 数据库配置
│   │   │   │   └── server.rs     # 服务器配置
│   │   │   ├── logging.rs        # 日志功能
│   │   │   ├── response/         # API响应格式
│   │   │   │   ├── mod.rs
│   │   │   │   ├── api_response.rs
│   │   │   │   ├── domain.rs
│   │   │   │   ├── error.rs
│   │   │   │   ├── reason.rs
│   │   │   │   └── status.rs
│   │   │   ├── state.rs          # 应用状态
│   │   │   ├── middleware/       # 中间件
│   │   │   │   ├── mod.rs
│   │   │   │   └── auth.rs       # JWT认证中间件
│   │   │   └── mod.rs
│   │   ├── error/                # 错误处理
│   │   │   ├── mod.rs
│   │   │   └── auth.rs           # 认证错误
│   │   ├── modules/              # 业务模块
│   │   │   ├── user/             # 用户管理模块
│   │   │   │   ├── mod.rs        # 路由定义
│   │   │   │   ├── handler.rs    # 请求处理器
│   │   │   │   ├── service.rs    # 业务逻辑
│   │   │   │   └── dto.rs        # 数据传输对象
│   │   │   └── docs/             # 文档模块
│   │   ├── routes/               # API路由
│   │   │   └── v1/               # V1版本API
│   │   ├── shared/               # 共享工具
│   │   │   ├── mod.rs
│   │   │   ├── jwt.rs            # JWT服务
│   │   │   └── password.rs       # 密码哈希（Argon2）
│   │   └── lib.rs
│   ├── assets/                   # 静态文件
│   └── Cargo.toml
├── entity/                       # 数据库实体（SeaORM生成）
│   ├── src/
│   │   ├── lib.rs
│   │   ├── user.rs
│   │   └── enums/
│   │       └── user_status.rs    # 用户状态枚举
│   └── Cargo.toml
├── migration/                    # 数据库迁移
│   ├── src/
│   │   ├── lib.rs
│   │   └── m20220101_000001_create_user_table.rs
│   └── Cargo.toml
├── config.toml                   # 配置文件
├── config.dev.toml               # 开发环境配置
├── .env.example                  # 环境变量示例
├── Cargo.toml                    # Workspace配置
└── README.md
```

## 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/yenharvey/my-axum-starter.git
cd my-axum-starter
```

OR

```shell
cargo generate yenharvey/my-axum-starter

cargo generate yenharvey/my-axum-starter --name my-new-project
```

### 2. 环境配置

创建 `.env` 文件：

```bash
# 必需
DATABASE_URL=postgresql://postgres:your_password@localhost:5432/your_database
JWT_SECRET=your-jwt-secret-key

# 可选
REDIS_URL=redis://localhost:6379
APP_SERVER_HOST=127.0.0.1
APP_SERVER_PORT=3001
APP_LOGGING_LEVEL=info
```

### 3. 数据库设置

```bash
# 执行数据库迁移
sea-orm-cli migrate up
```

### 4. 运行

```bash
cargo run -p app
```

访问 http://localhost:3001

## 配置

### config.toml

```toml
[server]
host = "127.0.0.1"
port = 3001
timeout = 30

[database]
max_connections = 10
pool_timeout = 30

[logging]
level = "info"
format = "pretty"
```

## API 文档

debug 模式下访问：http://localhost:3001/docs

## API 测试指南

### 1. 用户注册

```bash
curl -X POST http://127.0.0.1:3001/v1/user/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123",
    "password_confirm": "password123"
  }'
```

**响应示例：**
```json
{
  "api_version": "1.0",
  "data": {
    "kind": "User",
    "id": "1",
    "username": "testuser",
    "email": "test@example.com"
  }
}
```

### 2. 用户登录

```bash
curl -X POST http://127.0.0.1:3001/v1/user/login \
  -H "Content-Type: application/json" \
  -d '{
    "username_or_email": "testuser",
    "password": "password123"
  }'
```

**响应示例：**
```json
{
  "api_version": "1.0",
  "data": {
    "kind": "User",
    "id": "1",
    "username": "testuser",
    "email": "test@example.com",
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "expires_in": 604800
  }
}
```

### 3. 获取当前用户信息

使用登录返回的 token，在 Authorization header 中以 Bearer 格式传递：

```bash
curl -X GET http://127.0.0.1:3001/v1/user/me \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

**响应示例：**
```json
{
  "api_version": "1.0",
  "data": {
    "kind": "User",
    "id": "1",
    "username": "testuser",
    "email": "test@example.com"
  }
}
```

### 4. 健康检查

```bash
curl http://127.0.0.1:3001/health
```

## 常用命令

```bash
# 运行应用
cargo run -p app

# 数据库迁移
sea-orm-cli migrate up

# 回滚最后一个迁移
sea-orm-cli migrate down

# 生成数据库模型实体
sea-orm-cli generate entity -o entity/src

# 创建新的迁移文件
sea-orm-cli migrate generate create_xxx_table
```

## 许可证

[MIT](LICENSE)
