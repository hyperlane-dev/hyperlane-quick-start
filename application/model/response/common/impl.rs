use super::*;

/// Implementation of `From<ApiResponseStatus>` for `i32`, converting response status to its numeric HTTP code.
impl From<ApiResponseStatus> for i32 {
    /// Converts an `ApiResponseStatus` into its corresponding numeric HTTP status code.
    ///
    /// # Arguments
    ///
    /// - `ApiResponseStatus` - The response status to convert.
    fn from(status: ApiResponseStatus) -> Self {
        match status {
            ApiResponseStatus::Success => 200,
            ApiResponseStatus::InvalidRequest => 400,
            ApiResponseStatus::Unauthorized => 401,
            ApiResponseStatus::Forbidden => 403,
            ApiResponseStatus::ResourceNotFound => 404,
            ApiResponseStatus::Conflict => 409,
            ApiResponseStatus::DatabaseError => 500,
            ApiResponseStatus::BusinessLogicError => 500,
            ApiResponseStatus::InternalServerError => 500,
            ApiResponseStatus::ExternalServiceError => 502,
            ApiResponseStatus::RateLimitExceeded => 429,
            ApiResponseStatus::RequestTimeout => 408,
        }
    }
}

/// Implementation of `Display` for `ApiResponseStatus`, providing a human-readable status message.
impl Display for ApiResponseStatus {
    /// Formats the `ApiResponseStatus` as a human-readable status message string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message: &str = match self {
            Self::Success => HTTP_REASON_SUCCESS,
            Self::InvalidRequest => ERROR_INVALID_REQUEST,
            Self::Unauthorized => ERROR_UNAUTHORIZED,
            Self::Forbidden => ERROR_FORBIDDEN,
            Self::ResourceNotFound => ERROR_RESOURCE_NOT_FOUND,
            Self::Conflict => ERROR_CONFLICT,
            Self::DatabaseError => ERROR_DATABASE,
            Self::BusinessLogicError => ERROR_BUSINESS_LOGIC,
            Self::InternalServerError => ERROR_INTERNAL_SERVER,
            Self::ExternalServiceError => ERROR_EXTERNAL_SERVICE,
            Self::RateLimitExceeded => ERROR_RATE_LIMIT_EXCEEDED,
            Self::RequestTimeout => ERROR_REQUEST_TIMEOUT,
        };
        write!(f, "{}", message)
    }
}

/// Implementation of `ApiResponse` methods for constructing and serializing API responses.
impl<T> ApiResponse<T>
where
    T: Clone + Default + Serialize,
{
    /// Creates a new `ApiResponse` with the given status and data payload.
    ///
    /// # Arguments
    ///
    /// - `ApiResponseStatus` - The response status indicating the result of the operation.
    /// - `T` - The data payload to include in the response.
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

    /// Attempts to serialize the response to a JSON string.
    ///
    /// # Returns
    ///
    /// - `serde_json::Result<String>` - The JSON string representation or a serialization error.
    #[instrument_trace]
    pub fn try_to_json_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Serializes the response to a JSON string, returning an empty string on failure.
    ///
    /// # Returns
    ///
    /// - `String` - The JSON string representation of the response.
    #[instrument_trace]
    pub fn to_json_string(&self) -> String {
        self.try_to_json_string().unwrap_or_default()
    }

    /// Attempts to serialize the response to a JSON byte vector.
    ///
    /// # Returns
    ///
    /// - `serde_json::Result<Vec<u8>>` - The JSON byte vector or a serialization error.
    #[instrument_trace]
    pub fn try_to_json_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    /// Serializes the response to a JSON byte vector, returning an empty vector on failure.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The JSON byte vector representation of the response.
    #[instrument_trace]
    pub fn to_json_bytes(&self) -> Vec<u8> {
        self.try_to_json_bytes().unwrap_or_default()
    }
}
