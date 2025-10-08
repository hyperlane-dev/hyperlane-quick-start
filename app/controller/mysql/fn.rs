use super::*;

#[utoipa::path(
    get,
    path = "/mysql",
    description = "",
    responses(
        (status = 200, description = "", body = String)
    )
)]
#[route("/mysql")]
#[prologue_macros(get)]
pub async fn handle(ctx: Context) {}
