# 设计文档

## 概述

本设计文档描述了为 Hyperlane Web 服务器添加静态文件路由功能的技术实现方案。该功能将创建一个新的控制器来处理以 `/static` 开头的请求，并将这些请求映射到 `resources/static` 目录下的相应文件。

基于对现有代码的分析，项目已经有一个 `/static/{upload_dir}/{upload_file}` 路由用于处理上传文件的静态服务，我们将创建一个新的路由 `/static/{path:.*}` 来处理 `resources/static` 目录下的文件。

## 架构

### 整体架构
```
请求: /static/css/style.css
    ↓
路由匹配: /static/{path:.*}
    ↓
静态文件控制器 (StaticFilesController)
    ↓
静态文件服务 (StaticFileService)
    ↓
文件系统: resources/static/css/style.css
```

### 与现有系统的集成
- 复用现有的 `FileExtension` 和 `ContentType` 工具类
- 复用现有的文件读取和错误处理机制
- 遵循现有的控制器和服务层架构模式
- 使用现有的缓存和安全机制

## 组件和接口

### 1. 静态文件控制器 (StaticFilesController)
**位置**: `app/controller/static_files/`

**职责**:
- 处理 `/static/{path:.*}` 路由请求
- 参数验证和路径安全检查
- 调用静态文件服务
- 设置响应头和状态码

**接口**:
```rust
#[route("/static/{path:.*}")]
pub async fn serve_static_resource(ctx: Context)
```

### 2. 静态文件服务 (StaticFileService)
**位置**: `app/service/static_files/`

**职责**:
- 文件路径解析和验证
- 安全检查（防止路径遍历）
- 文件读取和内容类型检测
- 缓存控制和条件请求处理

**接口**:
```rust
pub async fn serve_resource_file(path: &str) -> Result<StaticFileResponse, StaticFileError>
pub fn validate_path_security(path: &str) -> Result<PathBuf, SecurityError>
pub fn get_content_type_for_file(file_path: &Path) -> String
```

### 3. 数据模型

**StaticFileResponse**:
```rust
pub struct StaticFileResponse {
    pub content: Vec<u8>,
    pub content_type: String,
    pub last_modified: Option<SystemTime>,
    pub etag: Option<String>,
}
```

**StaticFileError**:
```rust
pub enum StaticFileError {
    NotFound,
    Forbidden,
    SecurityViolation,
    IoError(std::io::Error),
}
```

## 数据模型

### 文件路径处理
- **输入路径**: `/static/css/style.css`
- **提取路径**: `css/style.css`
- **物理路径**: `resources/static/css/style.css`
- **安全验证**: 确保路径不包含 `../` 等危险字符

### 缓存机制
- **ETag**: 基于文件内容或修改时间生成
- **Last-Modified**: 文件的最后修改时间
- **Cache-Control**: 设置适当的缓存策略
- **条件请求**: 支持 `If-Modified-Since` 和 `If-None-Match`

## 错误处理

### 错误类型和响应
1. **文件不存在** (404 Not Found)
   - 请求的文件在 `resources/static` 目录中不存在
   - 返回标准的 404 错误页面

2. **路径安全违规** (400 Bad Request)
   - 包含 `../` 路径遍历尝试
   - 包含绝对路径
   - 包含特殊字符或编码绕过尝试

3. **访问被禁止** (403 Forbidden)
   - 尝试访问目录而非文件
   - 路径解析后超出允许范围

4. **服务器内部错误** (500 Internal Server Error)
   - 文件读取 I/O 错误
   - 系统级别的错误

### 错误处理流程
```rust
match serve_resource_file(&path).await {
    Ok(response) => {
        // 设置成功响应
        ctx.set_response_body(&response.content).await;
        ctx.set_response_header("Content-Type", &response.content_type).await;
        // 设置缓存头
    }
    Err(StaticFileError::NotFound) => {
        ctx.set_response_status_code(404).await;
    }
    Err(StaticFileError::SecurityViolation) => {
        ctx.set_response_status_code(400).await;
    }
    // 其他错误处理...
}
```

## 测试策略

### 单元测试
1. **路径验证测试**
   - 测试正常路径解析
   - 测试路径遍历攻击防护
   - 测试特殊字符处理

2. **文件服务测试**
   - 测试各种文件类型的正确服务
   - 测试 Content-Type 设置
   - 测试文件不存在的处理

3. **缓存机制测试**
   - 测试 ETag 生成和验证
   - 测试条件请求处理
   - 测试缓存头设置

### 集成测试
1. **端到端路由测试**
   - 测试完整的请求-响应流程
   - 测试与现有路由的兼容性
   - 测试性能和并发处理

2. **安全测试**
   - 测试各种路径遍历攻击
   - 测试权限和访问控制
   - 测试恶意请求处理

### 性能测试
1. **文件服务性能**
   - 测试大文件服务性能
   - 测试并发请求处理
   - 测试缓存效果

2. **内存使用测试**
   - 测试大文件处理时的内存使用
   - 测试长时间运行的稳定性

## 安全考虑

### 路径遍历防护
```rust
pub fn validate_path_security(path: &str) -> Result<PathBuf, SecurityError> {
    // 1. 检查是否包含危险字符
    if path.contains("../") || path.contains("..\\") {
        return Err(SecurityError::PathTraversal);
    }
    
    // 2. 规范化路径
    let normalized = Path::new("resources/static").join(path);
    let canonical = normalized.canonicalize()
        .map_err(|_| SecurityError::InvalidPath)?;
    
    // 3. 确保路径在允许范围内
    let base_path = Path::new("resources/static").canonicalize()
        .map_err(|_| SecurityError::InvalidBasePath)?;
    
    if !canonical.starts_with(base_path) {
        return Err(SecurityError::OutOfBounds);
    }
    
    Ok(canonical)
}
```

### 访问控制
- 只允许访问 `resources/static` 目录下的文件
- 禁止访问目录列表
- 禁止访问隐藏文件（以 `.` 开头的文件）

### 输入验证
- URL 解码后的路径验证
- 文件名长度限制
- 特殊字符过滤

## 实现细节

### 文件类型检测
复用现有的 `FileExtension` 工具类：
```rust
let extension_name = FileExtension::get_extension_name(&file_name);
let content_type = FileExtension::parse(&extension_name).get_content_type();
```

### 缓存策略
- **静态资源缓存**: 设置较长的缓存时间（如 1 年）
- **ETag 生成**: 基于文件修改时间和大小
- **条件请求**: 支持 `If-Modified-Since` 和 `If-None-Match`

### 性能优化
- **异步文件读取**: 使用 `tokio::fs` 进行异步 I/O
- **流式传输**: 对于大文件使用流式传输
- **内存管理**: 避免将大文件完全加载到内存

## 配置

### 新增配置常量
```rust
// config/business/static_files/const.rs
pub const STATIC_RESOURCES_DIR: &str = "resources/static";
pub const STATIC_ROUTE_PREFIX: &str = "static";
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
pub const CACHE_MAX_AGE: u32 = 31536000; // 1 year
```

### 路由优先级
确保新的 `/static/{path:.*}` 路由不与现有的 `/static/{upload_dir}/{upload_file}` 路由冲突，通过路由顺序和模式匹配来区分。