use super::*;

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
