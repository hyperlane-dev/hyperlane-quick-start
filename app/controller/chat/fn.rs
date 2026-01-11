use super::*;

#[utoipa::path(
    get,
    path = "/chat/users/online",
    responses(
        (status = 200, description = "Success", body = UserListResponse),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_get_online_users() {}
