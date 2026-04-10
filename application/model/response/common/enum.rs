use super::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum ApiResponseStatus {
    Success,
    InvalidRequest,
    Unauthorized,
    Forbidden,
    ResourceNotFound,
    DatabaseError,
    BusinessLogicError,
    InternalServerError,
    ExternalServiceError,
    RateLimitExceeded,
    RequestTimeout,
}
