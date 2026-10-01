use super::*;

/// openapi static resource view.
#[utoipa::path(
    get,
    path = ROUTE_STATIC_RESOURCE,
    params(
        ("path" = String, Path, description = ROUTE_PARAM_DESC_STATIC_PATH)
    ),
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_static_resource_view() {}
