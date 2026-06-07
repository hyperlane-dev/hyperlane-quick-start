use super::*;

/// openapi auth rsa public key.
#[utoipa::path(
    get,
    path = "/api/auth/rsa/public-key",
    responses(
        (status = 200, description = "RSA public key retrieved successfully"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_auth_rsa_public_key() {}

/// openapi auth register.
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

/// openapi auth login.
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

/// openapi auth user update.
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

/// openapi auth user change password.
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

/// openapi auth user update status.
#[utoipa::path(
    post,
    path = "/api/auth/user/update_status/{id}",
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
pub fn openapi_auth_user_update_status() {}

/// openapi auth user list.
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

/// openapi auth user get.
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

/// openapi auth user delete.
#[utoipa::path(
    post,
    path = "/api/auth/user/delete/{id}",
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
pub fn openapi_auth_user_delete() {}
