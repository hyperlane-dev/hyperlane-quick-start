use super::*;

#[route("/trace")]
#[utoipa::path(
    get,
    post,
    path = "/trace",
    description = "",
    responses(
        (status = 200, description = "", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(TRACE_HTML)
)]
pub async fn monitor_dashboard(ctx: Context) {}
