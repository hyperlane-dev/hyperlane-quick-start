use super::*;

#[utoipa::path(
    get,
    path = "/static/{path}",
    params(
        ("path" = String, Path, description = "Static resource path")
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_static_resource_view() {}
