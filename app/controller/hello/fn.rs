use super::*;

#[utoipa::path(
    get,
    path = "/hello/{name}",
    params(
        ("name" = String, Path, description = "User name")
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_hello_name() {}
