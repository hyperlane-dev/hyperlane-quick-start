use super::*;

#[utoipa::path(
    post,
    path = "/api/auth/register",
    responses(
        (status = 200, description = "User registered successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_auth_register() {}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    responses(
        (status = 200, description = "User logged in successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_auth_login() {}

#[utoipa::path(
    post,
    path = "/api/auth/user/update/{id}",
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
pub fn openapi_auth_user_update() {}

#[utoipa::path(
    post,
    path = "/api/auth/user/change_password/{id}",
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
pub fn openapi_auth_user_change_password() {}

#[utoipa::path(
    post,
    path = "/api/auth/user/approve/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User approval status updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_auth_user_approve() {}

#[utoipa::path(
    get,
    path = "/api/auth/user/list",
    responses(
        (status = 200, description = "List of users retrieved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_auth_user_list() {}

#[utoipa::path(
    get,
    path = "/api/auth/user/get/{id}",
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
pub fn openapi_auth_user_get() {}
