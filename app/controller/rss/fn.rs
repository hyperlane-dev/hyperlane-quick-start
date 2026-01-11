use super::*;

#[utoipa::path(
    get,
    path = "/api/rss/feed",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_rss_feed() {
    trace!("openapi_rss_feed");
}
