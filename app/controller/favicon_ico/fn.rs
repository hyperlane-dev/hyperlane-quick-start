use super::*;

#[get]
#[utoipa::path(
    get,
    path = "/favicon.ico",   
    responses(
        (status = 200, description = "Icon", body = String)
    )
)]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => IMAGE_PNG)]
#[response_header(CACHE_CONTROL => CACHE_CONTROL_SHORT_TERM)]
#[response_body(LOGO_IMG)]
pub async fn handle(ctx: Context) {}
