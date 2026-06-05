use super::*;

#[utoipa::path(
    post,
    path = "/api/github/pages/add",
    request_body = AddGithubPagesRequest,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_add() {}

#[utoipa::path(
    get,
    path = "/api/github/pages/list",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_list() {}

#[utoipa::path(
    get,
    path = "/api/github/pages/{owner}/{repository}",
    params(
        ("owner" = String, Path, description = "GitHub owner or organization name"),
        ("repository" = String, Path, description = "GitHub repository name")
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_resources() {}

#[utoipa::path(
    post,
    path = "/api/github/pages/delete/{id}",
    params(
        ("id" = String, Path, description = "GitHub Pages record ID")
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_delete() {}

#[utoipa::path(
    post,
    path = "/api/github/pages/sync",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_sync() {}
