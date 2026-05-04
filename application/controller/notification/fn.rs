use super::*;

#[utoipa::path(
    post,
    path = "/api/notification/create",
    responses(
        (status = 200, description = "Notification created successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_notification_create() {}

#[utoipa::path(
    get,
    path = "/api/notification/list",
    responses(
        (status = 200, description = "List of notifications retrieved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_notification_list() {}

#[utoipa::path(
    get,
    path = "/api/notification/get/{id}",
    params(
        ("id" = i32, Path, description = "Notification ID")
    ),
    responses(
        (status = 200, description = "Notification details retrieved successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Notification not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_notification_get() {}

#[utoipa::path(
    post,
    path = "/api/notification/read/{id}",
    params(
        ("id" = i32, Path, description = "Notification ID")
    ),
    responses(
        (status = 200, description = "Notification marked as read successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Notification not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_notification_read() {}

#[utoipa::path(
    post,
    path = "/api/notification/read-all",
    responses(
        (status = 200, description = "All notifications marked as read successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_notification_read_all() {}

#[utoipa::path(
    post,
    path = "/api/notification/delete/{id}",
    params(
        ("id" = i32, Path, description = "Notification ID")
    ),
    responses(
        (status = 200, description = "Notification deleted successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Notification not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_notification_delete() {}

#[utoipa::path(
    get,
    path = "/api/notification/unread-count",
    responses(
        (status = 200, description = "Unread count retrieved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_notification_unread_count() {}
