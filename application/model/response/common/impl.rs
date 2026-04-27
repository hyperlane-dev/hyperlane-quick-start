use super::*;

impl From<ApiResponseStatus> for i32 {
    fn from(status: ApiResponseStatus) -> Self {
        match status {
            ApiResponseStatus::Success => 200,
            ApiResponseStatus::InvalidRequest => 400,
            ApiResponseStatus::Unauthorized => 401,
            ApiResponseStatus::Forbidden => 403,
            ApiResponseStatus::ResourceNotFound => 404,
            ApiResponseStatus::DatabaseError => 500,
            ApiResponseStatus::BusinessLogicError => 500,
            ApiResponseStatus::InternalServerError => 500,
            ApiResponseStatus::ExternalServiceError => 502,
            ApiResponseStatus::RateLimitExceeded => 429,
            ApiResponseStatus::RequestTimeout => 408,
        }
    }
}

impl Display for ApiResponseStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message: &str = match self {
            Self::Success => "Success",
            Self::InvalidRequest => "Invalid request",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::ResourceNotFound => "Resource not found",
            Self::DatabaseError => "Database error",
            Self::BusinessLogicError => "Business logic error",
            Self::InternalServerError => "Internal server error",
            Self::ExternalServiceError => "External service error",
            Self::RateLimitExceeded => "Rate limit exceeded",
            Self::RequestTimeout => "Request timeout",
        };
        write!(f, "{}", message)
    }
}

impl<T> ApiResponse<T>
where
    T: Clone + Default + Serialize,
{
    #[instrument_trace]
    pub fn new(status: ApiResponseStatus, data: T) -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(status.into())
            .set_message(status.to_string())
            .set_data(Some(data))
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn new_with_message(status: ApiResponseStatus, message: String, data: T) -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(status.into())
            .set_message(message)
            .set_data(Some(data))
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn new_error(status: ApiResponseStatus, message: String) -> Self
    where
        T: From<String>,
    {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(status.into())
            .set_message(message.clone())
            .set_data(Some(T::from(message)))
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn try_to_json_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    #[instrument_trace]
    pub fn to_json_string(&self) -> String {
        self.try_to_json_string().unwrap_or_default()
    }

    #[instrument_trace]
    pub fn try_to_json_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    #[instrument_trace]
    pub fn to_json_bytes(&self) -> Vec<u8> {
        self.try_to_json_bytes().unwrap_or_default()
    }
}
