# 统一静态资源系统

## 概述

新的统一静态资源系统将所有静态资源的处理集中到一个路由中，支持多种资源类型，提供统一的缓存、安全和性能优化策略。

## 特性

### 🚀 统一路由处理
- 单一路由 `/{resource_type}/{path:.*}` 处理所有静态资源
- 支持多种资源类型：`static`、`assets`、`uploads`、`public`
- 自动路径验证和安全检查

### 🛡️ 安全防护
- 路径遍历攻击防护
- 隐藏文件访问限制
- 文件大小限制
- 路径长度验证

### ⚡ 性能优化
- HTTP 缓存支持（ETag、Last-Modified）
- 条件请求处理（304 Not Modified）
- HTTP 范围请求支持（206 Partial Content）
- 内容类型自动检测

### 📁 多资源类型支持
- **static**: 通用静态资源 (`resources/static/`)
- **assets**: 前端资源 (`resources/assets/`)
- **uploads**: 上传文件 (`uploads/`)
- **public**: 公共资源 (`resources/public/`)

## 文件结构规范

项目采用新的文件命名规范，目录名表示功能，文件名表示类型：

```
app/
├── controller/
│   └── static_files/
│       ├── fn.rs          # 路由处理函数
│       ├── struct.rs      # 数据结构定义
│       ├── impl.rs        # 实现逻辑
│       └── mod.rs         # 模块导出
├── service/
│   └── static_files/
│       ├── fn.rs          # 服务函数
│       ├── struct.rs      # 服务结构体
│       ├── impl.rs        # 服务实现
│       ├── const.rs       # 常量定义
│       └── mod.rs         # 模块导出
└── model/
    └── business/
        └── static_files/
            ├── struct.rs  # 业务结构体
            ├── enum.rs    # 枚举定义
            └── mod.rs     # 模块导出
```

## 使用方法

### 1. 基本用法

```rust
// 访问静态CSS文件
GET /static/css/style.css

// 访问前端JavaScript文件
GET /assets/js/app.js

// 访问上传的图片
GET /uploads/images/photo.jpg

// 访问公共文档
GET /public/docs/manual.pdf
```

### 2. 配置资源类型

```rust
use hyperlane_app::*;

// 创建不同类型的资源配置
let static_config = ResourceConfig::static_resources();
let assets_config = ResourceConfig::assets();
let uploads_config = ResourceConfig::uploads();
let public_config = ResourceConfig::public_resources();
```

### 3. 自定义缓存策略

```rust
// 长期缓存（1年）
let long_cache = CacheStrategy::LongTerm;

// 短期缓存（1小时）
let short_cache = CacheStrategy::ShortTerm;

// 自定义缓存时间
let custom_cache = CacheStrategy::Custom(7200); // 2小时

// 无缓存
let no_cache = CacheStrategy::NoCache;
```

## API 文档

### 路由端点

#### `GET /{resource_type}/{path:.*}`

统一的静态资源服务端点。

**参数:**
- `resource_type`: 资源类型 (`static`, `assets`, `uploads`, `public`)
- `path`: 文件路径

**请求头:**
- `If-Modified-Since`: 条件请求，检查文件是否已修改
- `If-None-Match`: ETag 条件请求
- `Range`: HTTP 范围请求

**响应:**
- `200 OK`: 成功返回文件内容
- `206 Partial Content`: 范围请求成功
- `304 Not Modified`: 文件未修改
- `400 Bad Request`: 请求参数错误
- `403 Forbidden`: 访问被禁止
- `404 Not Found`: 文件不存在
- `413 Payload Too Large`: 文件过大
- `416 Range Not Satisfiable`: 范围请求无效
- `500 Internal Server Error`: 服务器内部错误

**响应头:**
- `Content-Type`: 文件MIME类型
- `Content-Length`: 文件大小
- `Last-Modified`: 最后修改时间
- `ETag`: 实体标签
- `Cache-Control`: 缓存控制
- `Expires`: 过期时间
- `Accept-Ranges`: 支持范围请求

## 配置选项

### 资源类型配置

| 资源类型 | 基础目录 | 最大文件大小 | 缓存策略 |
|---------|----------|-------------|----------|
| static  | resources/static | 100MB | 长期缓存 |
| assets  | resources/assets | 50MB  | 长期缓存 |
| uploads | uploads          | 500MB | 短期缓存 |
| public  | resources/public | 10MB  | 自定义缓存 |

### 安全配置

```rust
// 最大路径长度
const MAX_PATH_LENGTH: usize = 255;

// 危险字符列表
const DANGEROUS_CHARS: &[char] = &['<', '>', ':', '"', '|', '?', '*', '\0'];

// Windows保留文件名
const WINDOWS_RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", // ...
];
```

## 性能特性

### 缓存机制
- **ETag**: 基于文件内容和修改时间生成
- **Last-Modified**: 文件最后修改时间
- **Cache-Control**: 根据文件类型设置不同的缓存策略
- **条件请求**: 支持 `If-Modified-Since` 和 `If-None-Match`

### 范围请求
- 支持 HTTP Range 请求
- 适用于大文件下载和媒体流
- 返回 `206 Partial Content` 状态码

### 内容类型检测
自动检测文件类型并设置正确的 `Content-Type` 头：

```rust
// 支持的文件类型
.css    -> text/css; charset=utf-8
.js     -> application/javascript; charset=utf-8
.html   -> text/html; charset=utf-8
.png    -> image/png
.jpg    -> image/jpeg
.json   -> application/json; charset=utf-8
// ... 更多类型
```

## 安全考虑

### 路径遍历防护
- 检测 `../` 和 `..\\` 模式
- 验证路径规范化后仍在允许范围内
- 禁止访问隐藏文件（以 `.` 开头）

### 文件大小限制
- 不同资源类型有不同的大小限制
- 防止过大文件消耗服务器资源

### 访问控制
- 只允许访问配置的基础目录
- 禁止目录列表（除非明确允许）
- 验证文件名安全性

## 错误处理

系统提供详细的错误信息和适当的HTTP状态码：

```rust
// 错误类型
pub enum StaticFileError {
    NotFound,           // 404
    Forbidden,          // 403
    SecurityViolation,  // 400
    FileTooLarge,      // 413
    InvalidPath,       // 400
    IoError(String),   // 500
}
```

## 监控和统计

### 文件统计
- 总文件数和大小
- 按文件类型分组的统计
- 平均文件大小计算

### 缓存清理
- 自动清理过期文件
- 可配置的清理间隔和策略

## 示例代码

### 基本使用

```rust
use hyperlane_app::*;

// 创建资源配置
let config = ResourceConfig::static_resources();

// 服务文件
let response = serve_unified_resource_file(&config, "css/style.css").await?;

// 检查文件是否存在
let exists = file_exists_unified(&config, "js/app.js").await;

// 获取文件信息
let (size, modified, etag) = get_file_info_unified(&config, "images/logo.png").await?;
```

### 自定义配置

```rust
let custom_config = ResourceConfig {
    resource_type: "custom".to_string(),
    base_dir: "custom/path".to_string(),
    allow_directory_listing: false,
    max_file_size: 50 * 1024 * 1024, // 50MB
    cache_strategy: CacheStrategy::Custom(3600), // 1小时
};
```

## 迁移指南

从旧的静态文件系统迁移到新系统：

1. **更新路由**: 将多个静态文件路由合并为统一路由
2. **更新配置**: 使用新的资源配置结构
3. **更新文件结构**: 按照新的命名规范重组文件
4. **测试**: 验证所有静态资源访问正常

## 最佳实践

1. **文件组织**: 按功能和类型组织静态文件
2. **缓存策略**: 为不同类型的文件设置合适的缓存时间
3. **安全检查**: 定期审查文件访问权限和路径安全
4. **性能监控**: 监控文件访问频率和响应时间
5. **定期清理**: 清理不再使用的文件和过期缓存

## 故障排除

### 常见问题

1. **文件无法访问**: 检查文件路径和权限
2. **缓存问题**: 验证ETag和Last-Modified头设置
3. **性能问题**: 检查文件大小和并发访问数
4. **安全警告**: 审查路径验证和访问控制

### 调试工具

```rust
// 启用详细日志
RUST_LOG=debug cargo run

// 检查文件统计
let stats = calculate_directory_stats(&config).await?;
println!("文件统计: {:?}", stats);

// 验证路径安全
match validate_unified_path_security(&config, path) {
    Ok(safe_path) => println!("安全路径: {:?}", safe_path),
    Err(error) => println!("安全错误: {:?}", error),
}
```

## 贡献

欢迎提交问题报告和功能请求。在提交代码时，请确保：

1. 遵循项目的文件命名规范
2. 添加适当的测试用例
3. 更新相关文档
4. 确保安全性和性能

## 许可证

本项目采用 MIT 许可证。详见 LICENSE 文件。