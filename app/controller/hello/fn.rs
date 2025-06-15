use super::*;

#[get]
#[utoipa::path(
    get,
    path = "/hello/{name}",   
    responses(
        (status = 200, description = "你好", body = String)
    )
)]
pub async fn handle(ctx: Context) {
    let name: String = ctx.get_route_param(NAME_KEY).await.unwrap_or_default();
    let _ = ctx
        .set_response_status_code(200)
        .await
        .set_response_body(format!("Hello {name}"))
        .await;
}
