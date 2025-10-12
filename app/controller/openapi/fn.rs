use super::*;

#[route("/openapi/openapi.json")]
#[utoipa::path(
    get,
    path = "/openapi/openapi.json",
    description = "Get OpenAPI specification data",
    responses(
        (status = 200, description = "Successfully retrieved OpenAPI specification", body = String)
    )
)]
#[prologue_macros(methods(get, post), response_status_code(200))]
pub async fn json(ctx: Context) {
    if let Ok(json_data) = ApiDoc::openapi().to_json() {
        let _ = ctx.set_response_body(&json_data).await;
    }
}
