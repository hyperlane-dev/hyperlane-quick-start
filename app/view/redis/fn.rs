use super::*;

#[route("/redis")]
#[utoipa::path(
    get,
    post,
    path = "/redis",
    description = "Redis frontend interface (redirects to static resource)",
    responses(
        (status = 302, description = "Redirect to static resource")
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(302),
    response_header(LOCATION => "/static/redis/index.html")
)]
pub async fn html(ctx: Context) {}
