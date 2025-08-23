use super::*;

#[utoipa::path(
    get,
    path = "/hello/{name}",   
    responses(
        (status = 200, description = "Hello", body = String)
    )
)]
#[get]
#[route("/hello/{name}")]
#[response_status_code(200)]
#[response_body(format!("Hello {}", name_opt.unwrap_or_default()))]
#[route_param(NAME_KEY => name_opt)]
pub async fn handle(ctx: Context) {}
