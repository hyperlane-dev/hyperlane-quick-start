use super::*;

#[post]
#[utoipa::path(
    get,
    path = "/hello/{name}",   
    responses(
        (status = 200, description = "Hello", body = String)
    )
)]
#[route("/hello/{name}")]
#[response_status_code(200)]
#[response_body(format!("Hello {}", name_opt.unwrap_or_default()))]
#[route_param(NAME_KEY => name_opt)]
pub async fn handle(ctx: Context) {}
