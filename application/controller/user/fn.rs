use super::*;

/// openapi user list.
#[utoipa::path(
    get,
    path = "/api/user/list",
    responses(
        (status = 200, description = "List of users retrieved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_list() {}

/// openapi user get.
#[utoipa::path(
    get,
    path = "/api/user/get/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details retrieved successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_get() {}

/// openapi user update.
#[utoipa::path(
    post,
    path = "/api/user/update/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_update() {}

/// openapi user change password.
#[utoipa::path(
    post,
    path = "/api/user/change_password/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_change_password() {}

/// openapi user update status.
#[utoipa::path(
    post,
    path = "/api/user/update_status/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User status updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_update_status() {}

/// openapi user delete.
#[utoipa::path(
    post,
    path = "/api/user/delete/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 400, description = "Bad request"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_delete() {}
