use super::*;

#[route("/monitor")]
#[utoipa::path(
    get,
    post,
    path = "/monitor",
    description = "Server monitoring dashboard interface (redirects to static resource)",
    responses(
        (status = 302, description = "Redirect to static resource")
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(302),
    response_header(LOCATION => "/static/monitor/index.html")
)]
pub async fn html(ctx: Context) {}
