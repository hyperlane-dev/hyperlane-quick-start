use super::*;

/// Renders the RSS feed page and serves as the OpenAPI documentation endpoint for RSS routes.
#[utoipa::path(
    get,
    path = "/rss",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_rss_view() {}
