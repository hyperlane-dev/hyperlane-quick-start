use super::*;

#[utoipa::path(
    get,
    path = "/openapi",
    responses(
        (status = 200, description = "OpenAPI documentation page")
    )
)]
pub async fn html() {}

impl ServerHook for OpenApiViewRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_HTML)
    )]
    async fn handle(self, ctx: &Context) {
        SwaggerUi::new("/openapi/{file}").url("/openapi/openapi.json", ApiDoc::openapi());
        let res: String =
            RapiDoc::with_openapi("/openapi/openapi.json", ApiDoc::openapi()).to_html();
        let _ = ctx.set_response_body(&res).await;
    }
}
