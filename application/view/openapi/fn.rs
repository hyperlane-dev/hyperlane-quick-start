use super::*;

/// Renders the OpenAPI specification viewer page.
#[utoipa::path(
    get,
    path = ROUTE_OPENAPI_ROOT,
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
/// Body of the `openapi_openapi_view` free function.
#[instrument_trace]
pub fn openapi_openapi_view() {}
