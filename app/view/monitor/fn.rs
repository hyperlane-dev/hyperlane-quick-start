use super::*;

#[route("/monitor")]
#[utoipa::path(
    get,
    post,
    path = "/monitor",
    description = "Server monitoring dashboard interface",
    responses(
        (status = 200, description = "Successfully served monitoring dashboard", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(MONITOR_DASHBOARD_HTML),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn handle(ctx: Context) {}
