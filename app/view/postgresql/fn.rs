use super::*;

#[route("/postgresql")]
#[utoipa::path(
    get,
    post,
    path = "/postgresql",
    description = "PostgreSQL frontend interface",
    responses(
        (status = 200, description = "Successfully served PostgreSQL frontend interface", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(POSTGRESQL_HTML),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn html(ctx: Context) {}
