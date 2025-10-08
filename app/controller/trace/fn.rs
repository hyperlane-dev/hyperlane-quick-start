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
    response_body(TRACE_HTML)
)]
pub async fn monitor_dashboard(ctx: Context) {}

#[route("/trace/{trace}")]
#[utoipa::path(
    get,
    path = "/trace/{trace}",
    description = "Search for a specific trace in log files",
    responses(
        (status = 200, description = "Successfully found trace", body = String),
        (status = 404, description = "Trace not found")
    ),
    params(
        ("trace" = String, description = "The trace ID to search for")
    )
)]
#[prologue_macros(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
    route_param("trace" => trace_opt)
)]
pub async fn trace(ctx: Context) {
    let trace: String = trace_opt.unwrap_or_default();
    let decoded_trace: String = decode(&trace)
        .unwrap_or_else(|_| trace.clone().into())
        .into_owned();
    let result: String = search_trace(&decoded_trace).await;
    ctx.set_response_body(&result).await;
}
