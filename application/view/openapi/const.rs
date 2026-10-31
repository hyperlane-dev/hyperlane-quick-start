/// OpenAPI route serving the documentation UI.
pub(crate) const ROUTE_OPENAPI_ROOT: &str = "/openapi";

/// Reason phrase for a successful response.
pub(crate) const HTTP_REASON_SUCCESS: &str = "Success";

/// Reason phrase for a malformed request.
pub(crate) const HTTP_REASON_BAD_REQUEST: &str = "Bad Request";

/// Reason phrase for a missing resource.
pub(crate) const HTTP_REASON_NOT_FOUND: &str = "Not Found";

/// Reason phrase for an unexpected server failure.
pub(crate) const HTTP_REASON_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// OpenAPI route serving one documentation file.
pub(crate) const ROUTE_OPENAPI_FILE: &str = "/openapi/{file}";

/// Path of the generated OpenAPI document.
pub(crate) const PATH_OPENAPI_SPEC: &str = "/openapi/openapi.json";
