use super::*;

/// Renders the authentication page and serves as the OpenAPI documentation endpoint for auth routes.
#[utoipa::path(
    get,
    path = "/auth",
    responses(
        (status = 302, description = "Redirect to auth page"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_auth_view() {}
