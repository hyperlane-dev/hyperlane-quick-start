use super::*;

#[get]
#[utoipa::path(
    get,
    path = "/favicon.ico",   
    responses(
        (status = 200, description = "Icon", body = String)
    )
)]
#[get]
#[utoipa::path(
    get,
    path = "/favicon.ico",   
    responses(
        (status = 200, description = "Icon", body = String)
    )
)]
#[get]
#[route("/favicon.ico")]
#[response_status_code(301)]
#[response_header(LOCATION => LOGO_IMG_URL)]
pub async fn handle(ctx: Context) {}
