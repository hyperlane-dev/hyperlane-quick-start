use super::*;

/// OpenAPI documentation endpoint for the health check route.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_HEALTH,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SERVICE_HEALTHY),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_HEALTH
)]
#[instrument_trace]
pub fn openapi_health() {}
