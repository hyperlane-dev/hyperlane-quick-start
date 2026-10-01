use super::*;

/// openapi get online users.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_CHAT_USERS_ONLINE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS, body = UserListResponse),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_get_online_users() {}
