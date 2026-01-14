use super::*;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 302, description = "Redirect to external URL"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "index"
)]
#[instrument_trace]
pub fn openapi_index() {}
