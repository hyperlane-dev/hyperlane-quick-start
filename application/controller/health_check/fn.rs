use super::*;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "health"
)]
#[instrument_trace]
pub fn openapi_health_check() {}
