use super::*;

/// openapi record create.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_ORDER_RECORD_CREATE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_RECORD_CREATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_record_create() {}

/// openapi record list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_ORDER_RECORD_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_RECORD_LIST_RETRIEVED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_record_list() {}

/// openapi record get.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_ORDER_RECORD_GET,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_RECORD_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_RECORD_DETAILS_RETRIEVED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_RECORD_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_record_get() {}

/// openapi overview statistics.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_ORDER_OVERVIEW_STATISTICS,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_OVERVIEW_STATISTICS_RETRIEVED),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 403, description = OPENAPI_DESCRIPTION_FORBIDDEN),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_overview_statistics() {}

/// openapi image upload.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_ORDER_IMAGE_UPLOAD,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_IMAGE_UPLOADED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_image_upload() {}

/// openapi image list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_ORDER_IMAGE_LIST,
    params(
        ("record_id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_RECORD_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_IMAGE_LIST_RETRIEVED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_image_list() {}

/// openapi image download.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_ORDER_IMAGE_DOWNLOAD,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_IMAGE_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_IMAGE_DOWNLOADED),
        (status = 404, description = ERROR_IMAGE_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_image_download() {}
