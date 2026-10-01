use super::*;

/// openapi create pipeline.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_CICD_PIPELINE_CREATE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_PIPELINE_CREATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_create_pipeline() {}

/// openapi list pipelines.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_CICD_PIPELINE_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_PIPELINE_LIST_RETRIEVED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_list_pipelines() {}

/// openapi get pipeline.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_CICD_PIPELINE_GET,
    params(
        ("id" = i32, Query, description = OPENAPI_PARAM_DESCRIPTION_PIPELINE_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_PIPELINE_DETAILS),
        (status = 404, description = ERROR_PIPELINE_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_get_pipeline() {}

/// openapi trigger run.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_CICD_RUN_TRIGGER,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_RUN_TRIGGERED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_trigger_run() {}

/// openapi list runs.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_CICD_RUN_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_RUN_LIST_RETRIEVED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_list_runs() {}

/// openapi get run.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_CICD_RUN_GET,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_RUN_DETAILS),
        (status = 404, description = ERROR_RUN_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_get_run() {}

/// openapi get run detail.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_CICD_RUN_DETAIL,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_RUN_DETAILS_WITH_JOBS),
        (status = 404, description = ERROR_RUN_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_get_run_detail() {}

/// openapi update job.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_CICD_JOB_UPDATE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_JOB_STATUS_UPDATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_update_job() {}

/// openapi update step.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_CICD_STEP_UPDATE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_STEP_STATUS_UPDATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_update_step() {}

/// openapi cicd view.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_CICD,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_CICD_MANAGEMENT_PAGE),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_cicd_view() {}
