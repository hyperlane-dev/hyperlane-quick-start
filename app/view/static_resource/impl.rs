use super::*;

fn determine_content_type(path: &str) -> String {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8".to_string(),
        Some("css") => "text/css".to_string(),
        Some("js") => "application/javascript".to_string(),
        Some("json") => "application/json".to_string(),
        Some("png") => "image/png".to_string(),
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("gif") => "image/gif".to_string(),
        Some("svg") => "image/svg+xml".to_string(),
        Some("ico") => "image/x-icon".to_string(),
        Some("woff") => "font/woff".to_string(),
        Some("woff2") => "font/woff2".to_string(),
        Some("ttf") => "font/ttf".to_string(),
        Some("mp4") => "video/mp4".to_string(),
        Some("webm") => "video/webm".to_string(),
        Some("ogg") => "video/ogg".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

impl ServerHook for StaticResourceRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        route_param(PATH_KEY => path_opt)
    )]
    async fn handle(self, ctx: &Context) {
        let path: String = path_opt.unwrap_or_default();
        if path.contains("..") || path.starts_with("/") || path.starts_with("\\") {
            ctx.set_response_status_code(403).await;
            return;
        }
        let file_path: String = format!("{}/{}", STATIC_RESOURCES_DIR, path);
        let canonical_path = match std::fs::canonicalize(&file_path) {
            Ok(p) => p,
            Err(_) => {
                ctx.set_response_status_code(404).await;
                return;
            }
        };
        let base_canonical = match std::fs::canonicalize(STATIC_RESOURCES_DIR) {
            Ok(p) => p,
            Err(_) => {
                ctx.set_response_status_code(500).await;
                return;
            }
        };
        if !canonical_path.starts_with(&base_canonical) {
            ctx.set_response_status_code(403).await;
            return;
        }
        match std::fs::read(&file_path) {
            Ok(content) => {
                let content_type = determine_content_type(&path);
                ctx.set_response_body(&content)
                    .await
                    .set_response_status_code(200)
                    .await
                    .set_response_header(CONTENT_TYPE, &content_type)
                    .await
                    .set_response_header(CACHE_CONTROL, CACHE_CONTROL_STATIC_ASSETS)
                    .await
                    .set_response_header(EXPIRES, EXPIRES_FAR_FUTURE)
                    .await;
            }
            Err(_) => {
                ctx.set_response_status_code(404).await;
            }
        }
    }
}
