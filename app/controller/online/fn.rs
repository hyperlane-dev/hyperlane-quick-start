use super::*;

#[utoipa::path(
    get,
    path = "/docs/online",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_online_docs() {
    trace!("openapi_online_docs");
}
