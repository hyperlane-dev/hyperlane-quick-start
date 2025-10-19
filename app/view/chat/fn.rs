use super::*;

#[route("/chat")]
#[utoipa::path(
    get,
    post,
    path = "/chat",
    description = "Chat frontend interface (redirects to static resource)",
    responses(
        (status = 302, description = "Redirect to static resource")
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(302),
    response_header(LOCATION => "/static/chat/index.html")
)]
pub async fn html(ctx: Context) {}
