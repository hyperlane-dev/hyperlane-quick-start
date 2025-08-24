use super::*;

#[route("/openapi/openapi.json")]
#[utoipa::path(
    get,
    path = "/openapi/openapi.json",
    responses(
        (status = 200, description = "OpenAPI data", body = String)
    )
)]
#[prologue_hooks[
    methods(get, post),
    response_status_code(200)
]]
pub async fn json(ctx: Context) {
    if let Ok(json_data) = ApiDoc::openapi().to_json() {
        let _ = ctx.set_response_body(json_data).await.send().await;
    }
}

#[route("/openapi")]
#[utoipa::path(
    get,
    path = "/openapi",
    responses(
        (status = 200, description = "OpenAPI documentation", body = String)
    )
)]
#[prologue_hooks[
    methods(get, post),
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_HTML)
]]
pub async fn html(ctx: Context) {
    SwaggerUi::new("/openapi/{file}").url("/openapi/openapi.json", ApiDoc::openapi());
    let res: String = RapiDoc::with_openapi("/openapi/openapi.json", ApiDoc::openapi()).to_html();
    let _ = ctx.set_response_body(res).await.send().await;
}
