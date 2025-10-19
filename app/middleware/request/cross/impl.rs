use super::*;

impl ServerHook for CrossMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_version(HttpVersion::HTTP1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(ACCESS_CONTROL_ALLOW_METHODS => ALL_METHODS)]
    #[response_header(ACCESS_CONTROL_ALLOW_HEADERS => WILDCARD_ANY)]
    async fn handle(self, ctx: &Context) {}
}
