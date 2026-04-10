use super::*;

impl ResponseCode {
    #[instrument_trace]
    pub fn default_message(&self) -> &'static str {
        match self {
            Self::Success => "Operation successful",
            Self::BadRequest => "Invalid request parameters",
            Self::Unauthorized => "Unauthorized access",
            Self::Forbidden => "Access forbidden",
            Self::NotFound => "Resource not found",
            Self::InternalError => "Internal server error",
            Self::DatabaseError => "Database operation failed",
            Self::BusinessError => "Business logic error",
        }
    }
}

impl<T> ApiResponse<T>
where
    T: Clone + Default + Serialize,
{
    #[instrument_trace]
    pub fn default_success() -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::Success as i32)
            .set_message("Success")
            .set_data(None)
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn success(data: T) -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::Success as i32)
            .set_message("Success")
            .set_data(Some(data))
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn success_with_message<M>(data: T, message: M) -> Self
    where
        M: AsRef<str>,
    {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::Success as i32)
            .set_message(message)
            .set_data(Some(data))
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn default_error() -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::InternalError as i32)
            .set_message("Internal server error")
            .set_data(None)
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn error<M>(message: M) -> Self
    where
        M: AsRef<str>,
    {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::InternalError as i32)
            .set_message(message)
            .set_data(None)
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn error_with_code<M>(code: ResponseCode, message: M) -> Self
    where
        M: AsRef<str>,
    {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(code as i32)
            .set_message(message)
            .set_data(None)
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }

    #[instrument_trace]
    pub fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }
}

impl ApiResponse<()> {
    #[instrument_trace]
    pub fn success_without_data<M>(message: M) -> Self
    where
        M: AsRef<str>,
    {
        let mut instance: ApiResponse<()> = Self::default();
        instance
            .set_code(ResponseCode::Success as i32)
            .set_message(message)
            .set_data(None)
            .set_timestamp(Some(Utc::now().timestamp_millis()));
        instance
    }
}
