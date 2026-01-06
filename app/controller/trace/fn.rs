use super::*;

#[utoipa::path(
    get,
    path = "/api/trace/{trace}",
    params(
        ("trace" = String, Path, description = "Trace ID")
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_trace_search() {}
