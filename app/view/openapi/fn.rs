use super::*;

#[route("/openapi")]
#[utoipa::path(
    get,
    path = "/openapi",
    description = "OpenAPI documentation interface",
    responses(
        (status = 200, description = "Successfully served OpenAPI documentation", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_HTML)
)]
pub async fn handle(ctx: Context) {
    SwaggerUi::new("/openapi/{file}").url("/openapi/openapi.json", ApiDoc::openapi());
    let res: String = RapiDoc::with_openapi("/openapi/openapi.json", ApiDoc::openapi()).to_html();
    let _ = ctx.set_response_body(&res).await;
}
