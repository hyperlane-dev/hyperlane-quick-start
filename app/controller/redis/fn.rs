use super::*;

#[utoipa::path(
    get,
    path = "/redis",
    description = "",
    responses(
        (status = 200, description = "", body = String)
    )
)]
#[route("/redis")]
#[prologue_macros(get)]
pub async fn handle(ctx: Context) {}
