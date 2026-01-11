use super::*;

#[utoipa::path(
    get,
    path = "/openapi/openapi.json",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_openapi_json() {
    trace!("openapi_openapi_json");
}
