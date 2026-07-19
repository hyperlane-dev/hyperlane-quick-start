use super::*;

/// openapi for the euv playground projects-list endpoint.
#[utoipa::path(
    get,
    path = "/api/euv/playground/projects",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "euv_playground",
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_list() {}

/// openapi for the euv playground projects-create endpoint.
#[utoipa::path(
    post,
    path = "/api/euv/playground/projects/create",
    request_body = EuvPlaygroundProjectCreateRequest,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Project name already exists"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "euv_playground",
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_create() {}

/// openapi for the euv playground projects-get endpoint.
#[utoipa::path(
    get,
    path = "/api/euv/playground/projects/get/{id}",
    params(
        ("id" = i64, Path, description = "Project id")
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "euv_playground",
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_get() {}

/// openapi for the euv playground projects-save endpoint.
#[utoipa::path(
    put,
    path = "/api/euv/playground/projects/save/{id}",
    params(
        ("id" = i64, Path, description = "Project id")
    ),
    request_body = EuvPlaygroundProjectSaveRequest,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 409, description = "Project name already exists"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "euv_playground",
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_save() {}

/// openapi for the euv playground projects-delete endpoint.
#[utoipa::path(
    delete,
    path = "/api/euv/playground/projects/delete/{id}",
    params(
        ("id" = i64, Path, description = "Project id")
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "euv_playground",
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_projects_delete() {}

/// openapi for the euv playground run endpoint.
#[utoipa::path(
    post,
    path = "/api/euv/playground/run",
    request_body = EuvPlaygroundRunRequest,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "euv_playground",
    security(
        ("cookie_auth" = [])
    )
)]
#[instrument_trace]
pub fn openapi_euv_playground_run() {}

/// openapi for the euv playground default-code endpoint.
#[utoipa::path(
    get,
    path = "/api/euv/playground/default-code",
    responses(
        (status = 200, description = "Success"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "euv_playground"
)]
#[instrument_trace]
pub fn openapi_euv_playground_default_code() {}
