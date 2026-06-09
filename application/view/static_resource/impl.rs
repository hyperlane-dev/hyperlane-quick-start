use super::*;

/// Implementation of `StaticResourceRoute` for `ServerHook`.
impl ServerHook for StaticResourceRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        try_get_route_param(PATH_KEY => path_opt)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let path: String = path_opt.unwrap_or_default();
        if path.contains("..") || path.starts_with("/") || path.starts_with("\\") {
            ctx.get_mut_response().set_status_code(403);
            return Status::Continue;
        }
        let file_path: String = format!("{STATIC_RESOURCES_DIR}/{path}");
        let canonical_path: PathBuf = match fs::canonicalize(&file_path) {
            Ok(p) => p,
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
                return Status::Continue;
            }
        };
        let base_canonical: PathBuf = match fs::canonicalize(STATIC_RESOURCES_DIR) {
            Ok(p) => p,
            Err(_) => {
                ctx.get_mut_response().set_status_code(500);
                return Status::Continue;
            }
        };
        if !canonical_path.starts_with(&base_canonical) {
            ctx.get_mut_response().set_status_code(403);
            return Status::Continue;
        }
        match fs::read(&file_path) {
            Ok(content) => {
                let extension: String = FileExtension::get_extension_name(&path);
                let content_type: &'static str =
                    FileExtension::parse(&extension).get_content_type();
                let response: &mut Response = ctx
                    .get_mut_response()
                    .set_body(&content)
                    .set_status_code(200)
                    .set_header(CONTENT_TYPE, content_type)
                    .set_header(CACHE_CONTROL, NO_CACHE_NO_STORE_MUST_REVALIDATE)
                    .set_header(PRAGMA, NO_CACHE)
                    .set_header(EXPIRES, EXPIRES_DISABLED);
                if is_gzip_compressible(&extension) {
                    response.set_header(CONTENT_ENCODING, GZIP);
                }
            }
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
            }
        }
        Status::Continue
    }
}
