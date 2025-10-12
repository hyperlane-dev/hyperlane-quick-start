# Implementation Plan

- [x] 1. 修改 MySQL 数据库初始化 schema
  - 更新 `plugin/mysql/impl.rs` 中的 `get_mysql_schema()` 方法
  - 移除 `hyperlane_config`、`hyperlane_logs`、`hyperlane_sessions` 表的定义
  - 添加 `mysql_record` 表的 SQL 定义，包含 `id`（主键，自增）、`key`（唯一索引）、`value` 字段
  - 确保使用 utf8mb4 字符集和 utf8mb4_unicode_ci 排序规则
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [x] 2. 修改 PostgreSQL 数据库初始化 schema
  - 更新 `plugin/postgresql/impl.rs` 中的 `get_postgresql_schema()` 方法
  - 移除 `hyperlane_config`、`hyperlane_logs`、`hyperlane_sessions` 表的定义
  - 添加 `postgresql_record` 表的 SQL 定义，包含 `id`（主键，SERIAL）、`key`（唯一约束）、`value` 字段
  - 确保使用 UTF8 编码
  - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [ ] 3. 验证 Mapper 层表名配置
  - 检查 `app/mapper/mysql/struct.rs` 中的 `table_name` 属性是否为 "mysql_record"
  - 检查 `app/mapper/postgresql/struct.rs` 中的 `table_name` 属性是否为 "postgresql_record"
  - 确保 Mapper 层的字段定义与 SQL schema 一致
  - _Requirements: 3.1, 3.2, 3.3_

- [ ] 4. 验证 Service 层数据库操作
  - 检查 `app/service/mysql/fn.rs` 中的所有数据库操作是否使用正确的 Entity
  - 检查 `app/service/postgresql/fn.rs` 中的所有数据库操作是否使用正确的 Entity
  - 确认 SeaORM 查询会自动使用 Entity 中定义的表名
  - _Requirements: 3.1, 3.2, 3.3_

- [ ] 5. 创建集成测试脚本
  - 创建测试脚本验证 MySQL 表创建和 CRUD 操作
  - 创建测试脚本验证 PostgreSQL 表创建和 CRUD 操作
  - 测试重复 key 的错误处理
  - 测试不存在记录的更新/删除操作
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7, 4.8_

- [ ] 6. 创建迁移文档
  - 编写现有系统的迁移步骤文档
  - 包含数据备份命令
  - 包含旧表删除命令
  - 包含验证步骤
  - _Requirements: 5.1, 5.2, 5.3, 5.4_
