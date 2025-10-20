use super::*;

#[utoipa::path(
    get,
    path = "/openapi/openapi.json",
    responses(
        (status = 200, description = "OpenAPI JSON specification")
    )
)]
pub async fn json() {}

impl ServerHook for OpenApiRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post), response_status_code(200))]
    async fn handle(self, ctx: &Context) {
        if let Ok(json_data) = ApiDoc::openapi().to_json() {
            let _ = ctx.set_response_body(&json_data).await;
        }
    }
}
