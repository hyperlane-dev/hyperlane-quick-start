use super::*;

#[get]
#[utoipa::path(
    get,
    path = "/hello/{name}",   
    responses(
        (status = 200, description = "你好", body = String)
    )
)]
#[route_param(NAME_KEY => name_opt)]
#[status_code(200)]
pub async fn handle(ctx: Context) {
    let name: String = name_opt.unwrap_or_default();
    let _ = ctx.set_response_body(format!("Hello {name}")).await;
}
