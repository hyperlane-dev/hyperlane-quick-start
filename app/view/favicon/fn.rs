use super::*;

#[utoipa::path(
    get,
    path = "/favicon.ico",
    description = "Favicon for the application",
    responses(
        (status = 200, description = "Application icon", body = String)
    )
)]
#[route("/favicon.ico")]
#[prologue_macros(
  get,
  response_status_code(301),
  response_header(LOCATION => LOGO_IMG_URL)
)]
pub async fn ico(ctx: Context) {}
