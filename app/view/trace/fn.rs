use super::*;

#[route("/trace")]
#[utoipa::path(
    get,
    post,
    path = "/trace",
    description = "Render the trace monitoring dashboard UI",
    responses(
        (status = 200, description = "Returns the HTML content of the trace monitoring dashboard", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(TRACE_HTML),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn html(ctx: Context) {}
