use super::*;

#[utoipa::path(
    post,
    path = "/api/account_booking/user/register",
    responses(
        (status = 200, description = "User registered successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_register() {}

#[utoipa::path(
    post,
    path = "/api/account_booking/user/login",
    responses(
        (status = 200, description = "User logged in successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_login() {}

#[utoipa::path(
    post,
    path = "/api/account_booking/user/create",
    responses(
        (status = 200, description = "User created successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_create() {}

#[utoipa::path(
    post,
    path = "/api/account_booking/user/update/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_update() {}

#[utoipa::path(
    post,
    path = "/api/account_booking/user/change_password/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_change_password() {}

#[utoipa::path(
    post,
    path = "/api/account_booking/user/approve/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User approval status updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_approve() {}

#[utoipa::path(
    get,
    path = "/api/account_booking/user/list",
    responses(
        (status = 200, description = "List of users retrieved successfully"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_user_list() {}

#[utoipa::path(
    get,
    path = "/api/account_booking/user/get/{id}",
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

#[utoipa::path(
    post,
    path = "/api/account_booking/record/create",
    responses(
        (status = 200, description = "Record created successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_record_create() {}

#[utoipa::path(
    post,
    path = "/api/account_booking/record/update/{id}",
    params(
        ("id" = i32, Path, description = "Record ID")
    ),
    responses(
        (status = 200, description = "Record updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Record not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_record_update() {}

#[utoipa::path(
    get,
    path = "/api/account_booking/record/list",
    responses(
        (status = 200, description = "List of records retrieved successfully"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_record_list() {}

#[utoipa::path(
    get,
    path = "/api/account_booking/record/get/{id}",
    params(
        ("id" = i32, Path, description = "Record ID")
    ),
    responses(
        (status = 200, description = "Record details retrieved successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Record not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_record_get() {}
