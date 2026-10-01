/// OpenAPI route serving the documentation UI.
pub const ROUTE_OPENAPI_ROOT: &str = "/openapi";

/// Reason phrase for a successful response.
pub const HTTP_REASON_SUCCESS: &str = "Success";

/// Reason phrase for a malformed request.
pub const HTTP_REASON_BAD_REQUEST: &str = "Bad Request";

/// Reason phrase for a missing resource.
pub const HTTP_REASON_NOT_FOUND: &str = "Not Found";

/// Reason phrase for an unexpected server failure.
pub const HTTP_REASON_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// OpenAPI route serving one documentation file.
pub const ROUTE_OPENAPI_FILE: &str = "/openapi/{file}";

/// Path of the generated OpenAPI document.
pub const PATH_OPENAPI_SPEC: &str = "/openapi/openapi.json";
