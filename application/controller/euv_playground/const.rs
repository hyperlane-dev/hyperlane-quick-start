/// Error returned when the request URL is missing the `{id}` project
/// path parameter (e.g. `PUT /api/euv/playground/projects/save/` with
/// no trailing id segment).
pub const ERROR_MISSING_PROJECT_ID: &str = "missing project id";

/// Error returned when the `{id}` project path parameter is not a
/// valid `i64` after URL-decoding.
pub const ERROR_INVALID_PROJECT_ID: &str = "project id is not a number";

/// Error returned when the decoded project id does not exist under
/// the caller's `data/euv_playground/{user_id}/` directory. This is
/// distinct from "you do not own this project" — that case is rejected
/// before the lookup so the caller cannot enumerate other users'
/// project ids via 404 vs 403 timing.
pub const ERROR_PROJECT_NOT_FOUND: &str = "project not found";

/// Prefix of the error returned when the requested project name
/// collides with an existing project in the caller's directory. The
/// caller emits
/// `format!("{ERROR_PROJECT_NAME_TAKEN_PREFIX}{}{ERROR_PROJECT_NAME_TAKEN_SUFFIX}", name)`.
pub const ERROR_PROJECT_NAME_TAKEN_PREFIX: &str = "Project name \"";

/// Suffix of the collision error. Pairs with
/// [`ERROR_PROJECT_NAME_TAKEN_PREFIX`] and the offending name in the
/// middle.
pub const ERROR_PROJECT_NAME_TAKEN_SUFFIX: &str = "\" already exists";

/// Error returned when the project file could not be removed during a
/// `DELETE /api/euv/playground/projects/delete/{id}` request. `{}` is
/// the underlying `std::io::Error`.
pub const ERROR_DELETE_PROJECT_FAILED: &str = "Failed to delete project: {}";

/// Error returned when the build job id path segment is missing from
/// the `GET /api/euv/playground/run/status/{id}` URL.
pub const ERROR_MISSING_JOB_ID: &str = "missing job id";

/// Error returned when the build job id path segment is not a valid
/// `u64` after URL-decoding.
pub const ERROR_INVALID_JOB_ID: &str = "job id is not valid";

/// Error returned when no in-memory build job matches the requested
/// id (either expired and GC'd, or never existed).
pub const ERROR_JOB_NOT_FOUND: &str = "job not found";

/// Prefix of the "code exceeds" validation error. The caller emits
/// `format!("{PREFIX}{} bytes (got {})", max, code_len)` so the byte
/// cap and the actual size sit in the middle of the message.
pub const ERROR_CODE_EXCEEDS_PREFIX: &str = "code exceeds ";

/// Project name to fall back to when the request body submits an empty
/// or whitespace-only name. Surfaced in the sidebar and echoed back in
/// the response payload so the UI is never blank.
///
/// Intentionally duplicated from
/// `service::euv_playground::UNTITLED_PROJECT_NAME`: a controller-level
/// glob-import would otherwise cause a name collision with this file's
/// own `r#const::*` re-export. Keeping the value in sync is a one-line
/// audit (`rg UNTITLED_PROJECT_NAME`).
pub const CONTROLLER_UNTITLED_PROJECT_NAME: &str = "Untitled";

/// OpenAPI response description for bad request.
pub const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad Request";

/// OpenAPI response description for euv job not found.
pub const OPENAPI_DESCRIPTION_EUV_JOB_NOT_FOUND: &str = "Job not found";

/// OpenAPI response description for euv project name exists.
pub const OPENAPI_DESCRIPTION_EUV_PROJECT_NAME_EXISTS: &str = "Project name already exists";

/// OpenAPI response description for euv project not found.
pub const OPENAPI_DESCRIPTION_EUV_PROJECT_NOT_FOUND: &str = "Project not found";

/// OpenAPI response description for internal server error.
pub const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// OpenAPI response description for success.
pub const OPENAPI_DESCRIPTION_SUCCESS: &str = "Success";

/// OpenAPI response description for unauthorized.
pub const OPENAPI_DESCRIPTION_UNAUTHORIZED: &str = "Unauthorized";

/// OpenAPI description of the build job id parameter.
pub const OPENAPI_PARAM_DESCRIPTION_BUILD_JOB_ID: &str = "Build job id returned by /run";

/// OpenAPI description of the project id parameter.
pub const OPENAPI_PARAM_DESCRIPTION_PROJECT_ID: &str = "Project id";

/// OpenAPI route path for the euv playground default code endpoint.
pub const OPENAPI_PATH_EUV_PLAYGROUND_DEFAULT_CODE: &str = "/api/euv/playground/default-code";

/// OpenAPI route path for the euv playground projects endpoint.
pub const OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS: &str = "/api/euv/playground/projects";

/// OpenAPI route path for the euv playground projects create endpoint.
pub const OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS_CREATE: &str = "/api/euv/playground/projects/create";

/// OpenAPI route path for the euv playground projects delete endpoint.
pub const OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS_DELETE: &str =
    "/api/euv/playground/projects/delete/{id}";

/// OpenAPI route path for the euv playground projects get endpoint.
pub const OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS_GET: &str = "/api/euv/playground/projects/get/{id}";

/// OpenAPI route path for the euv playground projects save endpoint.
pub const OPENAPI_PATH_EUV_PLAYGROUND_PROJECTS_SAVE: &str =
    "/api/euv/playground/projects/save/{id}";

/// OpenAPI route path for the euv playground run endpoint.
pub const OPENAPI_PATH_EUV_PLAYGROUND_RUN: &str = "/api/euv/playground/run";

/// OpenAPI route path for the euv playground run status endpoint.
pub const OPENAPI_PATH_EUV_PLAYGROUND_RUN_STATUS: &str = "/api/euv/playground/run/status/{id}";

/// OpenAPI tag grouping the euv playground routes.
pub const OPENAPI_TAG_EUV_PLAYGROUND: &str = "euv_playground";
