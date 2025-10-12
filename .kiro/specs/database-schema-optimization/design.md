# Design Document

## Overview

本设计文档描述了如何优化数据库初始化架构，使其与实际 API 接口使用的表结构保持一致。主要目标是修改 MySQL 和 PostgreSQL 的自动建表逻辑，移除未使用的表，添加实际需要的表，并确保所有数据库操作都正确指定表名。

## Architecture

### Current Architecture

当前系统使用以下架构：

1. **Plugin Layer** (`plugin/mysql`, `plugin/postgresql`, `plugin/redis`)
   - 负责数据库连接和初始化
   - 实现 `DatabaseAutoCreation` trait
   - 定义初始化 SQL schema

2. **Mapper Layer** (`app/mapper`)
   - 定义 SeaORM Entity 模型
   - 当前定义了 `mysql_record` 和 `postgresql_record` 表结构

3. **Service Layer** (`app/service`)
   - 实现业务逻辑
   - 使用 Mapper 层的 Entity 进行数据库操作

4. **Controller Layer** (`app/controller`)
   - 处理 HTTP 请求
   - 调用 Service 层方法

### Problem Analysis

**问题 1：表结构不匹配**
- 初始化 SQL 创建的表：`hyperlane_config`, `hyperlane_logs`, `hyperlane_sessions`
- API 实际使用的表：`mysql_record`, `postgresql_record`
- 结果：初始化的表未被使用，实际需要的表未被创建

**问题 2：表名隐式依赖**
- Mapper 层通过 `#[sea_orm(table_name = "...")]` 指定表名
- 但初始化 SQL 中没有创建这些表
- 可能导致运行时错误

### Proposed Solution

修改 Plugin 层的初始化逻辑，使其创建与 Mapper 层定义一致的表结构。

## Components and Interfaces

### 1. MySQL Plugin (`plugin/mysql/impl.rs`)

#### Modified Method: `get_mysql_schema()`

**Current Implementation:**
```rust
fn get_mysql_schema(&self) -> crate::database::DatabaseSchema {
    DatabaseSchema::new()
        .add_table(TableSchema::new("hyperlane_config", ...))
        .add_table(TableSchema::new("hyperlane_logs", ...))
        .add_table(TableSchema::new("hyperlane_sessions", ...))
}
```

**New Implementation:**
```rust
fn get_mysql_schema(&self) -> crate::database::DatabaseSchema {
    DatabaseSchema::new()
        .add_table(
            TableSchema::new(
                "mysql_record".to_string(),
                r#"CREATE TABLE `mysql_record` (
                    `id` INT NOT NULL AUTO_INCREMENT,
                    `key` VARCHAR(255) NOT NULL,
                    `value` TEXT,
                    PRIMARY KEY (`id`),
                    UNIQUE KEY `uk_key` (`key`)
                ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci"#.to_string(),
            )
        )
}
```

**Rationale:**
- 与 `app/mapper/mysql/struct.rs` 中的 Model 定义保持一致
- 使用 InnoDB 引擎以支持事务
- 使用 utf8mb4 字符集以支持完整的 Unicode 字符
- 在 `key` 字段上创建唯一索引以防止重复

### 2. PostgreSQL Plugin (`plugin/postgresql/impl.rs`)

#### Modified Method: `get_postgresql_schema()`

**Current Implementation:**
```rust
fn get_postgresql_schema(&self) -> crate::database::DatabaseSchema {
    DatabaseSchema::new()
        .add_table(TableSchema::new("hyperlane_config", ...))
        .add_table(TableSchema::new("hyperlane_logs", ...))
        .add_table(TableSchema::new("hyperlane_sessions", ...))
}
```

**New Implementation:**
```rust
fn get_postgresql_schema(&self) -> crate::database::DatabaseSchema {
    DatabaseSchema::new()
        .add_table(
            TableSchema::new(
                "postgresql_record".to_string(),
                r#"CREATE TABLE postgresql_record (
                    id SERIAL PRIMARY KEY,
                    key VARCHAR(255) NOT NULL UNIQUE,
                    value TEXT
                )"#.to_string(),
            )
        )
}
```

**Rationale:**
- 与 `app/mapper/postgresql/struct.rs` 中的 Model 定义保持一致
- 使用 SERIAL 类型实现自增主键
- 在 `key` 字段上创建唯一约束

### 3. Redis Plugin (No Changes Required)

Redis 不需要修改，因为：
- Redis 是键值存储，不需要预定义表结构
- 当前的初始化逻辑只是设置一些初始化标记键
- API 接口直接使用 Redis 命令操作数据

## Data Models

### MySQL Record Model

**Mapper Definition** (`app/mapper/mysql/struct.rs`):
```rust
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "mysql_record", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(unique)]
    pub key: String,
    pub value: String,
}
```

**SQL Schema**:
```sql
CREATE TABLE `mysql_record` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `key` VARCHAR(255) NOT NULL,
    `value` TEXT,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_key` (`key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci
```

**Field Mapping:**
- `id`: INT AUTO_INCREMENT → i32 (primary key)
- `key`: VARCHAR(255) UNIQUE → String (unique constraint)
- `value`: TEXT → String

### PostgreSQL Record Model

**Mapper Definition** (`app/mapper/postgresql/struct.rs`):
```rust
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "postgresql_record", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(unique)]
    pub key: String,
    pub value: String,
}
```

**SQL Schema**:
```sql
CREATE TABLE postgresql_record (
    id SERIAL PRIMARY KEY,
    key VARCHAR(255) NOT NULL UNIQUE,
    value TEXT
)
```

**Field Mapping:**
- `id`: SERIAL → i32 (primary key)
- `key`: VARCHAR(255) UNIQUE → String (unique constraint)
- `value`: TEXT → String

## Error Handling

### Initialization Error Handling

当前系统已经有完善的错误处理机制：

1. **Database Creation Errors**
   - `AutoCreationError::InsufficientPermissions`: 权限不足
   - `AutoCreationError::ConnectionFailed`: 连接失败
   - `AutoCreationError::DatabaseError`: 数据库错误

2. **Table Creation Errors**
   - `AutoCreationError::SchemaError`: Schema 错误
   - 表已存在时跳过创建

3. **Logging**
   - `AutoCreationLogger::log_table_created()`: 记录表创建成功
   - `AutoCreationLogger::log_table_exists()`: 记录表已存在
   - `AutoCreationLogger::log_auto_creation_error()`: 记录错误

### Service Layer Error Handling

Service 层已经实现了完善的错误处理：

```rust
pub async fn create_mysql_record(record: MysqlRecord) -> Result<(), String> {
    // ...
    active_model.insert(&db).await.map_err(|error: DbErr| error.to_string())?;
    Ok(())
}
```

所有数据库操作都返回 `Result<T, String>`，错误信息会传递到 Controller 层返回给客户端。

## Testing Strategy

### 1. Unit Testing (Optional)

可以为以下组件编写单元测试：
- Schema 生成逻辑
- 表名验证逻辑

### 2. Integration Testing

**测试场景：**

1. **数据库初始化测试**
   - 测试 MySQL 数据库和 `mysql_record` 表是否正确创建
   - 测试 PostgreSQL 数据库和 `postgresql_record` 表是否正确创建
   - 验证表结构（字段、类型、约束）

2. **API 功能测试**
   - 测试 `/api/mysql/create` 创建记录
   - 测试 `/api/mysql/list` 查询记录
   - 测试 `/api/mysql/update` 更新记录
   - 测试 `/api/mysql/delete` 删除记录
   - 对 PostgreSQL 和 Redis 执行相同测试

3. **错误处理测试**
   - 测试重复 key 的处理
   - 测试不存在记录的更新/删除
   - 测试数据库连接失败的处理

### 3. Manual Testing

**测试步骤：**

1. 清空现有数据库
2. 启动应用程序
3. 验证日志中的表创建信息
4. 使用数据库客户端检查表结构
5. 通过 API 接口测试 CRUD 操作
6. 验证数据持久化

## Migration Strategy

### For New Installations

新安装的系统会自动创建正确的表结构，无需额外操作。

### For Existing Installations

对于已有的系统，需要执行以下步骤：

1. **备份现有数据**（如果有）
   ```sql
   -- MySQL
   mysqldump -u username -p database_name > backup.sql
   
   -- PostgreSQL
   pg_dump -U username database_name > backup.sql
   ```

2. **删除旧表**（如果存在）
   ```sql
   -- MySQL
   DROP TABLE IF EXISTS hyperlane_config;
   DROP TABLE IF EXISTS hyperlane_logs;
   DROP TABLE IF EXISTS hyperlane_sessions;
   
   -- PostgreSQL
   DROP TABLE IF EXISTS hyperlane_config;
   DROP TABLE IF EXISTS hyperlane_logs;
   DROP TABLE IF EXISTS hyperlane_sessions;
   ```

3. **重启应用程序**
   - 系统会自动创建新的表结构

4. **验证表结构**
   ```sql
   -- MySQL
   SHOW CREATE TABLE mysql_record;
   
   -- PostgreSQL
   \d postgresql_record
   ```

## Implementation Notes

### 1. Table Name Consistency

确保以下位置的表名保持一致：
- Plugin 层的 SQL schema (`plugin/mysql/impl.rs`, `plugin/postgresql/impl.rs`)
- Mapper 层的 Entity 定义 (`app/mapper/mysql/struct.rs`, `app/mapper/postgresql/struct.rs`)

### 2. Character Set and Collation

**MySQL:**
- 使用 `utf8mb4` 字符集（支持完整 Unicode，包括 emoji）
- 使用 `utf8mb4_unicode_ci` 排序规则（不区分大小写，支持多语言）

**PostgreSQL:**
- 默认使用 UTF8 编码
- 不需要显式指定字符集

### 3. Index Strategy

**MySQL:**
- 主键索引：自动创建在 `id` 字段
- 唯一索引：在 `key` 字段上创建，防止重复

**PostgreSQL:**
- 主键索引：自动创建在 `id` 字段
- 唯一约束：在 `key` 字段上创建，防止重复

### 4. Backward Compatibility

修改后的代码保持向后兼容：
- 如果表已存在，跳过创建
- 如果数据库已存在，跳过创建
- 错误处理机制保持不变
- API 接口保持不变

## Security Considerations

1. **SQL Injection Prevention**
   - 使用 SeaORM 的参数化查询
   - 不直接拼接 SQL 字符串

2. **Permission Management**
   - 数据库用户需要 CREATE DATABASE 权限
   - 数据库用户需要 CREATE TABLE 权限
   - 错误日志会记录权限问题

3. **Data Validation**
   - Controller 层验证输入数据
   - 数据库层通过唯一约束防止重复

## Performance Considerations

1. **Index Usage**
   - `key` 字段的唯一索引提高查询性能
   - 主键索引提高按 ID 查询的性能

2. **Connection Pooling**
   - SeaORM 自动管理连接池
   - 无需额外配置

3. **Query Optimization**
   - 使用 SeaORM 的查询构建器
   - 避免 N+1 查询问题

## Monitoring and Logging

系统已有完善的日志记录：

1. **Initialization Logs**
   - 数据库创建日志
   - 表创建日志
   - 连接验证日志

2. **Error Logs**
   - 权限错误
   - 连接错误
   - Schema 错误

3. **Operation Logs**
   - CRUD 操作可以通过应用日志追踪
