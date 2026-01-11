use super::*;

impl ServerHook for StaticResourceRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        route_param_option(PATH_KEY => path_opt)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let path: String = path_opt.unwrap_or_default();
        if path.contains("..") || path.starts_with("/") || path.starts_with("\\") {
            ctx.set_response_status_code(403).await;
            return;
        }
        let file_path: String = format!("{STATIC_RESOURCES_DIR}/{path}");
        let canonical_path: PathBuf = match fs::canonicalize(&file_path) {
            Ok(p) => p,
            Err(_) => {
                ctx.set_response_status_code(404).await;
                return;
            }
        };
        let base_canonical: PathBuf = match fs::canonicalize(STATIC_RESOURCES_DIR) {
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
        match fs::read(&file_path) {
            Ok(content) => {
                let extension: String = FileExtension::get_extension_name(&path);
                let content_type: &'static str =
                    FileExtension::parse(&extension).get_content_type();
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
