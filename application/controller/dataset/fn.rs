use super::*;

/// OpenAPI documentation endpoint for the dataset fetching route.
#[utoipa::path(
    get,
    path = "/dataset",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_dataset() {}
