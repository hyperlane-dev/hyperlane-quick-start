use super::*;

/// openapi euv playground view.
#[utoipa::path(
    get,
    path = "/euv-playground",
    responses(
        (status = 302, description = "Redirect to euv playground SPA"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "euv_playground"
)]
#[instrument_trace]
pub fn openapi_euv_playground_view() {}
