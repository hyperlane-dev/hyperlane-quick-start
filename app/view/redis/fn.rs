use super::*;

#[route("/redis")]
#[utoipa::path(
    get,
    post,
    path = "/redis",
    description = "Redis frontend interface",
    responses(
        (status = 200, description = "Successfully served Redis frontend interface", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(REDIS_HTML),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn html(ctx: Context) {}
