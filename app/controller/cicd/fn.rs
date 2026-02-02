use super::*;

#[utoipa::path(
    post,
    path = "/api/cicd/pipeline/create",
    responses(
        (status = 200, description = "Pipeline created successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_create_pipeline() {}

#[utoipa::path(
    get,
    path = "/api/cicd/pipeline/list",
    responses(
        (status = 200, description = "List of pipelines"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_list_pipelines() {}

#[utoipa::path(
    get,
    path = "/api/cicd/pipeline/get",
    params(
        ("id" = i32, Query, description = "Pipeline ID")
    ),
    responses(
        (status = 200, description = "Pipeline details"),
        (status = 404, description = "Pipeline not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_get_pipeline() {}

#[utoipa::path(
    post,
    path = "/api/cicd/run/trigger",
    responses(
        (status = 200, description = "Run triggered successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_trigger_run() {}

#[utoipa::path(
    get,
    path = "/api/cicd/run/list",
    responses(
        (status = 200, description = "Paginated list of runs"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_list_runs() {}

#[utoipa::path(
    get,
    path = "/api/cicd/run/get",
    responses(
        (status = 200, description = "Run details"),
        (status = 404, description = "Run not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_get_run() {}

#[utoipa::path(
    get,
    path = "/api/cicd/run/detail",
    responses(
        (status = 200, description = "Run details with jobs and steps"),
        (status = 404, description = "Run not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_get_run_detail() {}

#[utoipa::path(
    post,
    path = "/api/cicd/job/update",
    responses(
        (status = 200, description = "Job status updated"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_update_job() {}

#[utoipa::path(
    post,
    path = "/api/cicd/step/update",
    responses(
        (status = 200, description = "Step status updated"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_update_step() {}

#[utoipa::path(
    get,
    path = "/cicd",
    responses(
        (status = 200, description = "CICD management page"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_cicd_view() {}
