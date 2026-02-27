use super::*;

impl ServerHook for StaticResourceRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        route_param_option(PATH_KEY => path_opt)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let path: String = path_opt.unwrap_or_default();
        if path.contains("..") || path.starts_with("/") || path.starts_with("\\") {
            ctx.get_mut_response().set_status_code(403);
            return;
        }
        let file_path: String = format!("{STATIC_RESOURCES_DIR}/{path}");
        let canonical_path: PathBuf = match fs::canonicalize(&file_path) {
            Ok(p) => p,
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
                return;
            }
        };
        let base_canonical: PathBuf = match fs::canonicalize(STATIC_RESOURCES_DIR) {
            Ok(p) => p,
            Err(_) => {
                ctx.get_mut_response().set_status_code(500);
                return;
            }
        };
        if !canonical_path.starts_with(&base_canonical) {
            ctx.get_mut_response().set_status_code(403);
            return;
        }
        match fs::read(&file_path) {
            Ok(content) => {
                let extension: String = FileExtension::get_extension_name(&path);
                let content_type: &'static str =
                    FileExtension::parse(&extension).get_content_type();
                ctx.get_mut_response()
                    .set_body(&content)
                    .set_status_code(200)
                    .set_header(CONTENT_TYPE, content_type)
                    .set_header(CACHE_CONTROL, CACHE_CONTROL_STATIC_ASSETS)
                    .set_header(EXPIRES, EXPIRES_FAR_FUTURE);
            }
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
            }
        }
    }
}
