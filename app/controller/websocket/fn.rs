use super::*;

#[utoipa::path(
    get,
    path = "/websocket",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_websocket() {
    trace!("openapi_websocket");
}
