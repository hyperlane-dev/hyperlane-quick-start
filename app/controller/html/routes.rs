use super::*;
use std::path::Path;
use crate::service::auth::AuthService;

#[route("/login")]
#[prologue_hooks(get)]
pub async fn serve_login_page(ctx: Context) {
    // 检查用户是否已经登录
    if let Some(session_id) = extract_session_from_request(&ctx).await {
        // 验证会话
        if let Ok(auth_service) = AuthService::from_global_pool() {
            if auth_service.validate_session(&session_id).is_ok() {
                // 用户已登录，重定向到首页
                ctx.set_response_status_code(302).await;
                ctx.set_response_header("Location", "/").await;
                return;
            }
        }
    }
    
    serve_html_file(&ctx, "resources/static/html/login.html").await;
}

#[route("/")]
#[prologue_hooks(get)]
pub async fn serve_index_page(ctx: Context) {
    serve_html_file(&ctx, "resources/templates/html/index.html").await;
}

#[route("/static/{file_path}")]
#[prologue_hooks(get)]
pub async fn serve_static_file(ctx: Context) {
    // 从路由参数中获取文件路径
    let route_params = ctx.get_route_params().await;
    let file_path = route_params.get("file_path").map_or("", |v| v.as_str());
    
    // 构建完整的文件路径
    let full_path = format!("resources/static/{}", file_path);
    
    // 安全检查：防止路径遍历攻击
    if file_path.contains("..") || file_path.starts_with('/') {
        ctx.set_response_status_code(403).await;
        ctx.set_response_header("Content-Type", "text/plain; charset=utf-8").await;
        ctx.set_response_body(b"Forbidden").await;
        return;
    }
    
    serve_file(&ctx, &full_path).await;
}

/// 通用HTML文件服务函数
async fn serve_html_file(ctx: &Context, file_path: &str) {
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            ctx.set_response_header("Content-Type", "text/html; charset=utf-8").await;
            ctx.set_response_status_code(200).await;
            ctx.set_response_body(content.as_bytes()).await;
        }
        Err(e) => {
            eprintln!("Failed to read HTML file {}: {}", file_path, e);
            serve_404_page(ctx).await;
        }
    }
}

/// 通用文件服务函数
async fn serve_file(ctx: &Context, file_path: &str) {
    match std::fs::read(file_path) {
        Ok(content) => {
            // 设置适当的 Content-Type
            let content_type = get_content_type(file_path);
            ctx.set_response_header("Content-Type", &content_type).await;
            ctx.set_response_status_code(200).await;
            ctx.set_response_body(&content).await;
        }
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path, e);
            serve_404_page(ctx).await;
        }
    }
}

/// 服务404页面
async fn serve_404_page(ctx: &Context) {
    ctx.set_response_status_code(404).await;
    
    // 尝试服务 404 页面
    match std::fs::read_to_string("resources/static/html/404.html") {
        Ok(not_found_content) => {
            ctx.set_response_header("Content-Type", "text/html; charset=utf-8").await;
            ctx.set_response_body(not_found_content.as_bytes()).await;
        }
        Err(_) => {
            // 如果连 404 页面都找不到，返回简单的文本响应
            ctx.set_response_header("Content-Type", "text/plain; charset=utf-8").await;
            ctx.set_response_body(b"404 Not Found").await;
        }
    }
}

/// 从请求中提取会话ID
async fn extract_session_from_request(_ctx: &Context) -> Option<String> {
    // TODO: 实现从 Cookie 或 Authorization 头中获取会话ID
    // 这里需要根据实际的 Context API 来实现
    None
}

/// 根据文件扩展名确定 Content-Type
fn get_content_type(file_path: &str) -> String {
    let path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html; charset=utf-8".to_string(),
        Some("css") => "text/css; charset=utf-8".to_string(),
        Some("js") => "application/javascript; charset=utf-8".to_string(),
        Some("json") => "application/json; charset=utf-8".to_string(),
        Some("png") => "image/png".to_string(),
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("gif") => "image/gif".to_string(),
        Some("svg") => "image/svg+xml".to_string(),
        Some("ico") => "image/x-icon".to_string(),
        Some("woff") => "font/woff".to_string(),
        Some("woff2") => "font/woff2".to_string(),
        Some("ttf") => "font/ttf".to_string(),
        Some("eot") => "application/vnd.ms-fontobject".to_string(),
        _ => "text/plain; charset=utf-8".to_string(),
    }
}