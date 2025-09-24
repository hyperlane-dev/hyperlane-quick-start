use super::*;

// 首页路由已移至统一的静态资源路由处理
// #[route("/")]
// #[prologue_hooks(get)]
// pub async fn serve_index_page(ctx: Context) {
//     serve_html_file(&ctx, "resources/templates/html/index.html").await;
// }

// 旧的静态文件路由已移除，现在使用统一的 /static/{path:.*} 路由

async fn serve_html_file(ctx: &Context, file_path: &str) {
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            ctx.set_response_header("Content-Type", "text/html; charset=utf-8")
                .await;
            ctx.set_response_status_code(200).await;
            ctx.set_response_body(content.as_bytes()).await;
        }
        Err(e) => {
            eprintln!("Failed to read HTML file {}: {}", file_path, e);
        }
    }
}
