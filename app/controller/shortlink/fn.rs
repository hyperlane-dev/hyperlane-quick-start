use super::*;

#[utoipa::path(
    get,
    path = "/api/shortlink/query/{id}",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_shortlink_query() {
    trace!("openapi_shortlink_query");
}

#[utoipa::path(
    post,
    path = "/api/shortlink/insert",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_shortlink_insert() {
    trace!("openapi_shortlink_insert");
}
