use super::*;

#[get]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => IMAGE_PNG)]
#[response_header(CACHE_CONTROL => "public, max-age=3600")]
#[response_body(LOGO_IMG)]
pub async fn handle(ctx: Context) {}
