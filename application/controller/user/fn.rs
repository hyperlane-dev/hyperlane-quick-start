use super::*;

/// openapi user list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_USER_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_USER_LIST_RETRIEVED),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_user_list() {}

/// openapi user get.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_USER_GET,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_USER_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_USER_DETAILS_RETRIEVED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_USER_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_user_get() {}

/// openapi user update.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_USER_UPDATE,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_USER_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_USER_UPDATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 403, description = OPENAPI_DESCRIPTION_FORBIDDEN),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_user_update() {}

/// openapi user change password.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_USER_CHANGE_PASSWORD,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_USER_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_USER_PASSWORD_CHANGED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_user_change_password() {}

/// openapi user update status.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_USER_UPDATE_STATUS,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_USER_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_USER_STATUS_UPDATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_user_update_status() {}

/// openapi user delete.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_USER_DELETE,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_USER_ID)
    ),
    responses(
        (status = 200, description = SUCCESS_USER_DELETED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 403, description = OPENAPI_DESCRIPTION_FORBIDDEN),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_user_delete() {}
