# Requirements Document

## Introduction

本需求旨在优化数据库初始化架构，确保初始化的数据表与实际 API 接口使用的表一致。当前系统在 MySQL 和 PostgreSQL 初始化时创建了一些未使用的表（`hyperlane_config`、`hyperlane_logs`、`hyperlane_sessions`），而实际 API 使用的表（`mysql_record`、`postgresql_record`）却没有在初始化时创建。此外，需要确保所有数据库查询操作都明确指定正确的表名。

## Requirements

### Requirement 1: 优化 MySQL 数据库初始化架构

**User Story:** 作为系统管理员，我希望 MySQL 数据库初始化时只创建实际使用的表，以便减少资源浪费并提高系统可维护性。

#### Acceptance Criteria

1. WHEN MySQL 数据库初始化时 THEN 系统 SHALL 创建 `mysql_record` 表，包含字段：`id`（主键，自增）、`key`（唯一索引）、`value`
2. WHEN MySQL 数据库初始化时 THEN 系统 SHALL NOT 创建 `hyperlane_config`、`hyperlane_logs`、`hyperlane_sessions` 表
3. WHEN `mysql_record` 表创建时 THEN 系统 SHALL 使用 utf8mb4 字符集和 utf8mb4_unicode_ci 排序规则
4. WHEN `mysql_record` 表创建时 THEN 系统 SHALL 在 `key` 字段上创建唯一索引

### Requirement 2: 优化 PostgreSQL 数据库初始化架构

**User Story:** 作为系统管理员，我希望 PostgreSQL 数据库初始化时只创建实际使用的表，以便减少资源浪费并提高系统可维护性。

#### Acceptance Criteria

1. WHEN PostgreSQL 数据库初始化时 THEN 系统 SHALL 创建 `postgresql_record` 表，包含字段：`id`（主键，自增）、`key`（唯一索引）、`value`
2. WHEN PostgreSQL 数据库初始化时 THEN 系统 SHALL NOT 创建 `hyperlane_config`、`hyperlane_logs`、`hyperlane_sessions` 表
3. WHEN `postgresql_record` 表创建时 THEN 系统 SHALL 使用 UTF8 编码
4. WHEN `postgresql_record` 表创建时 THEN 系统 SHALL 在 `key` 字段上创建唯一约束

### Requirement 3: 确保数据库查询操作明确指定表名

**User Story:** 作为开发人员，我希望所有数据库查询操作都明确指定表名，以便确保查询的准确性和可维护性。

#### Acceptance Criteria

1. WHEN 执行 MySQL 查询操作时 THEN 系统 SHALL 明确使用 `mysql_record` 表名
2. WHEN 执行 PostgreSQL 查询操作时 THEN 系统 SHALL 明确使用 `postgresql_record` 表名
3. WHEN 使用 SeaORM Entity 进行查询时 THEN 系统 SHALL 确保 Entity 定义中的 `table_name` 属性正确设置

### Requirement 4: 验证 API 接口功能完整性

**User Story:** 作为 API 用户，我希望在架构优化后所有 API 接口功能保持正常，以便确保系统稳定性。

#### Acceptance Criteria

1. WHEN 调用 `/api/mysql/list` 接口时 THEN 系统 SHALL 正确返回所有 MySQL 记录
2. WHEN 调用 `/api/mysql/create` 接口时 THEN 系统 SHALL 成功创建新记录到 `mysql_record` 表
3. WHEN 调用 `/api/mysql/update` 接口时 THEN 系统 SHALL 成功更新 `mysql_record` 表中的记录
4. WHEN 调用 `/api/mysql/delete` 接口时 THEN 系统 SHALL 成功删除 `mysql_record` 表中的记录
5. WHEN 调用 `/api/postgresql/list` 接口时 THEN 系统 SHALL 正确返回所有 PostgreSQL 记录
6. WHEN 调用 `/api/postgresql/create` 接口时 THEN 系统 SHALL 成功创建新记录到 `postgresql_record` 表
7. WHEN 调用 `/api/postgresql/update` 接口时 THEN 系统 SHALL 成功更新 `postgresql_record` 表中的记录
8. WHEN 调用 `/api/postgresql/delete` 接口时 THEN 系统 SHALL 成功删除 `postgresql_record` 表中的记录
9. WHEN 调用 Redis 相关接口时 THEN 系统 SHALL 保持现有功能不变

### Requirement 5: 保持向后兼容性

**User Story:** 作为系统维护者，我希望架构优化不会破坏现有的数据库连接和配置，以便平滑升级。

#### Acceptance Criteria

1. WHEN 系统启动时 THEN 系统 SHALL 正常连接到 MySQL、PostgreSQL 和 Redis 数据库
2. WHEN 数据库已存在时 THEN 系统 SHALL 跳过数据库创建步骤
3. WHEN 表已存在时 THEN 系统 SHALL 跳过表创建步骤
4. WHEN 初始化过程中出现错误时 THEN 系统 SHALL 记录详细的错误日志
