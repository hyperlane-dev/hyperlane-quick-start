use super::*;

/// openapi github pages list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_GITHUB_PAGES_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_list() {}

/// openapi github pages sync.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_GITHUB_PAGES_SYNC,
    params(
        ("owner" = String, Path, description = OPENAPI_PARAM_DESCRIPTION_OWNER),
        ("repository" = String, Path, description = OPENAPI_PARAM_DESCRIPTION_REPOSITORY)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_sync() {}
