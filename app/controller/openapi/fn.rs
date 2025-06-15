use super::*;

#[methods(get, post)]
#[utoipa::path(
    get,
    path = "/openapi/openapi.json",   
    responses(
        (status = 200, description = "Openapi数据", body = String)
    )
)]
pub async fn json(ctx: Context) {
    ctx.set_response_status_code(200)
        .await
        .set_response_body(ApiDoc::openapi().to_json().unwrap())
        .await
        .send()
        .await
        .unwrap();
}

#[methods(get, post)]
#[utoipa::path(
    get,
    path = "/openapi/index.html",   
    responses(
        (status = 200, description = "Openapi文档", body = String)
    )
)]
pub async fn html(ctx: Context) {
    SwaggerUi::new("/openapi/{file}").url("/openapi/openapi.json", ApiDoc::openapi());
    let res: String = RapiDoc::with_openapi("/openapi/openapi.json", ApiDoc::openapi()).to_html();
    ctx.set_response_status_code(200)
        .await
        .set_response_header(CONTENT_TYPE, TEXT_HTML)
        .await
        .set_response_body(res)
        .await
        .send()
        .await
        .unwrap();
}
