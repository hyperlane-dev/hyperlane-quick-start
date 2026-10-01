use super::*;

/// openapi notification create.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_NOTIFICATION_CREATE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_NOTIFICATION_CREATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_notification_create() {}

/// openapi notification list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_NOTIFICATION_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_NOTIFICATION_LIST_RETRIEVED),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_notification_list() {}

/// openapi notification get.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_NOTIFICATION_GET,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_NOTIFICATION_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_NOTIFICATION_DETAILS_RETRIEVED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOTIFICATION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_notification_get() {}

/// openapi notification read.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_NOTIFICATION_MARK_READ,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_NOTIFICATION_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_NOTIFICATION_MARKED_AS_READ),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOTIFICATION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_notification_read() {}

/// openapi notification read all.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_NOTIFICATION_READ_ALL,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_NOTIFICATION_ALL_MARKED_READ),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_notification_read_all() {}

/// openapi notification delete.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_NOTIFICATION_DELETE,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_NOTIFICATION_ID)
    ),
    responses(
        (status = 200, description = SUCCESS_NOTIFICATION_DELETED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOTIFICATION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_notification_delete() {}

/// openapi notification unread count.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_NOTIFICATION_UNREAD_COUNT,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_NOTIFICATION_UNREAD_COUNT_RETRIEVED),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_notification_unread_count() {}
