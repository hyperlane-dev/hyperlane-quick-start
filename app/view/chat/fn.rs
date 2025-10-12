use super::*;

#[route("/chat")]
#[utoipa::path(
    get,
    post,
    path = "/chat",
    description = "Chat frontend interface",
    responses(
        (status = 200, description = "Successfully served chat frontend interface", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(CHAT_HTML),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn handle(ctx: Context) {}
