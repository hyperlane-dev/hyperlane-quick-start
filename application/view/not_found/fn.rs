use super::*;

/// Renders the page that tells the visitor the address is unknown and serves as the OpenAPI documentation endpoint for the not found route.
#[utoipa::path(
    get,
    path = ROUTE_NOT_FOUND,
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_not_found_view() {}
