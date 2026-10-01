/// Reason phrase for a successful response.
pub const HTTP_REASON_SUCCESS: &str = "Success";

/// Message for an invalid request.
pub const ERROR_INVALID_REQUEST: &str = "Invalid request";

/// Message for an unauthenticated request.
pub const ERROR_UNAUTHORIZED: &str = "Unauthorized";

/// Message for a forbidden request.
pub const ERROR_FORBIDDEN: &str = "Forbidden";

/// Message for a missing resource.
pub const ERROR_RESOURCE_NOT_FOUND: &str = "Resource not found";

/// Message for a conflicting request.
pub const ERROR_CONFLICT: &str = "Conflict";

/// Message for a database failure.
pub const ERROR_DATABASE: &str = "Database error";

/// Message for a business rule failure.
pub const ERROR_BUSINESS_LOGIC: &str = "Business logic error";

/// Message for an internal server failure.
pub const ERROR_INTERNAL_SERVER: &str = "Internal server error";

/// Message for a downstream service failure.
pub const ERROR_EXTERNAL_SERVICE: &str = "External service error";

/// Message for a rate limit rejection.
pub const ERROR_RATE_LIMIT_EXCEEDED: &str = "Rate limit exceeded";

/// Message for a timed out request.
pub const ERROR_REQUEST_TIMEOUT: &str = "Request timeout";
