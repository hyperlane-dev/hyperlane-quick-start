/// OpenAPI route for a static resource.
pub(crate) const ROUTE_STATIC_RESOURCE: &str = "/static/{path}";

/// Description of the resource path route parameter.
pub(crate) const ROUTE_PARAM_DESC_STATIC_PATH: &str = "Static resource path";

/// Reason phrase for a malformed request.
pub(crate) const HTTP_REASON_BAD_REQUEST: &str = "Bad Request";

/// Reason phrase for an unexpected server failure.
pub(crate) const HTTP_REASON_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// Reason phrase for a missing resource.
pub(crate) const HTTP_REASON_NOT_FOUND: &str = "Not Found";

/// Reason phrase for a successful response.
pub(crate) const HTTP_REASON_SUCCESS: &str = "Success";
