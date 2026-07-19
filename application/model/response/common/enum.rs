use super::*;

/// Enumeration of API response status codes with associated semantic meaning.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum ApiResponseStatus {
    /// The request was processed successfully.
    Success,
    /// The request was invalid or malformed.
    InvalidRequest,
    /// The request lacked valid authentication credentials.
    Unauthorized,
    /// The authenticated user lacks permission for the requested resource.
    Forbidden,
    /// The requested resource was not found.
    ResourceNotFound,
    /// A resource conflicts with existing state.
    Conflict,
    /// A database operation failed.
    DatabaseError,
    /// A business logic constraint was violated.
    BusinessLogicError,
    /// An unexpected internal server error occurred.
    InternalServerError,
    /// An external service request failed.
    ExternalServiceError,
    /// The rate limit for the request was exceeded.
    RateLimitExceeded,
    /// The request timed out before completion.
    RequestTimeout,
}
