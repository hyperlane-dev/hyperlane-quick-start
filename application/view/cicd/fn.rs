use super::*;

/// Renders the CI/CD page and serves as the OpenAPI documentation endpoint for CI/CD routes.
#[utoipa::path(
    get,
    path = ROUTE_CICD,
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_cicd_view() {}
