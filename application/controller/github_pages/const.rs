/// OpenAPI response description for bad request.
pub const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad Request";

/// OpenAPI response description for internal server error.
pub const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// OpenAPI response description for not found.
pub const OPENAPI_DESCRIPTION_NOT_FOUND: &str = "Not Found";

/// OpenAPI response description for success.
pub const OPENAPI_DESCRIPTION_SUCCESS: &str = "Success";

/// OpenAPI description of the owner parameter.
pub const OPENAPI_PARAM_DESCRIPTION_OWNER: &str = "GitHub owner or organization name";

/// OpenAPI description of the repository parameter.
pub const OPENAPI_PARAM_DESCRIPTION_REPOSITORY: &str = "GitHub repository name";

/// OpenAPI route path for the github pages list endpoint.
pub const OPENAPI_PATH_GITHUB_PAGES_LIST: &str = "/api/github/pages/list";

/// OpenAPI route path for the github pages sync endpoint.
pub const OPENAPI_PATH_GITHUB_PAGES_SYNC: &str = "/api/github/pages/sync/{owner}/{repository}";
