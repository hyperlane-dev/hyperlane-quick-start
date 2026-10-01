use super::*;

/// Renders the authentication page and serves as the OpenAPI documentation endpoint for auth routes.
#[utoipa::path(
    get,
    path = ROUTE_AUTH,
    responses(
        (status = 302, description = RESPONSE_DESCRIPTION_REDIRECT),
        (status = 500, description = RESPONSE_DESCRIPTION_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_auth_view() {}
