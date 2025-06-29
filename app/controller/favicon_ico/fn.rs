use super::*;

#[get]
#[utoipa::path(
    get,
    path = "/favicon.ico",   
    responses(
        (status = 200, description = "图标", body = String)
    )
)]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => IMAGE_PNG)]
#[response_header(CACHE_CONTROL => "public, max-age=3600")]
#[response_body(LOGO_IMG)]
pub async fn handle(ctx: Context) {}
