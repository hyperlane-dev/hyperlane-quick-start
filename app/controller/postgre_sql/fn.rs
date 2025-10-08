use super::*;

#[utoipa::path(
    get,
    path = "/postgre_sql",
    description = "",
    responses(
        (status = 200, description = "", body = String)
    )
)]
#[route("/postgre_sql")]
#[prologue_macros(get)]
pub async fn handle(ctx: Context) {}
