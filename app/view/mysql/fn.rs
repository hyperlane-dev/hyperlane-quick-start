use super::*;

#[route("/mysql")]
#[utoipa::path(
    get,
    post,
    path = "/mysql",
    description = "MySQL frontend interface (redirects to static resource)",
    responses(
        (status = 200, description = "Redirect to static resource", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(302),
   response_header(LOCATION => "/static/postgresql/index.html")
)]
pub async fn html(ctx: Context) {}
