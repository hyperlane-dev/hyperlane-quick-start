use super::*;

#[route("/mysql")]
#[utoipa::path(
    get,
    post,
    path = "/mysql",
    description = "MySQL frontend interface",
    responses(
        (status = 200, description = "Successfully served MySQL frontend interface", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(MYSQL_HTML),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn handle(ctx: Context) {}
