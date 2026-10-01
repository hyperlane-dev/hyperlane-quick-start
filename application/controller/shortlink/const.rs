/// Error message when shortlink id required.
pub const ERROR_SHORTLINK_ID_REQUIRED: &str = "Shortlink ID parameter is required";

/// Error message when shortlink not found.
pub const ERROR_SHORTLINK_NOT_FOUND: &str = "Shortlink not found";

/// OpenAPI response description for bad request.
pub const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad Request";

/// OpenAPI response description for internal server error.
pub const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// OpenAPI response description for not found.
pub const OPENAPI_DESCRIPTION_NOT_FOUND: &str = "Not Found";

/// OpenAPI response description for success.
pub const OPENAPI_DESCRIPTION_SUCCESS: &str = "Success";

/// OpenAPI route path for the shortlink insert endpoint.
pub const OPENAPI_PATH_SHORTLINK_INSERT: &str = "/api/shortlink/insert";

/// OpenAPI route path for the shortlink query endpoint.
pub const OPENAPI_PATH_SHORTLINK_QUERY: &str = "/api/shortlink/query/{id}";
