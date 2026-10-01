use super::*;

/// openapi for the euv playground projects-list endpoint.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_EUV_PLAYGROUND,
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_list() {}

/// openapi for the euv playground projects-create endpoint.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS_CREATE,
    request_body = EuvPlaygroundProjectCreateRequest,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 409, description = OPENAPI_DESCRIPTION_EUV_PROJECT_NAME_EXISTS),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_EUV_PLAYGROUND,
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_create() {}

/// openapi for the euv playground projects-get endpoint.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS_GET,
    params(
        ("id" = i64, Path, description = OPENAPI_PARAM_DESCRIPTION_PROJECT_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = OPENAPI_DESCRIPTION_EUV_PROJECT_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_EUV_PLAYGROUND,
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_get() {}

/// openapi for the euv playground projects-save endpoint.
#[utoipa::path(
    put,
    path = OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS_SAVE,
    params(
        ("id" = i64, Path, description = OPENAPI_PARAM_DESCRIPTION_PROJECT_ID)
    ),
    request_body = EuvPlaygroundProjectSaveRequest,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = OPENAPI_DESCRIPTION_EUV_PROJECT_NOT_FOUND),
        (status = 409, description = OPENAPI_DESCRIPTION_EUV_PROJECT_NAME_EXISTS),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_EUV_PLAYGROUND,
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_save() {}

/// openapi for the euv playground projects-delete endpoint.
#[utoipa::path(
    delete,
    path = OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS_DELETE,
    params(
        ("id" = i64, Path, description = OPENAPI_PARAM_DESCRIPTION_PROJECT_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = OPENAPI_DESCRIPTION_EUV_PROJECT_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_EUV_PLAYGROUND,
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_delete() {}

/// openapi for the euv playground run endpoint.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_EUV_PLAYGROUND_RUN,
    request_body = EuvPlaygroundRunRequest,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = OPENAPI_DESCRIPTION_EUV_PROJECT_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_EUV_PLAYGROUND,
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_run() {}

/// openapi for the euv playground run status endpoint.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_EUV_PLAYGROUND_RUN_STATUS,
    params(
        ("id" = u64, Path, description = OPENAPI_PARAM_DESCRIPTION_BUILD_JOB_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = OPENAPI_DESCRIPTION_EUV_JOB_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_EUV_PLAYGROUND,
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_run_status() {}

/// openapi for the euv playground default-code endpoint.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_EUV_PLAYGROUND_DEFAULT_CODE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_EUV_PLAYGROUND
)]
#[instrument_trace]
pub fn openapi_euv_playground_default_code() {}
