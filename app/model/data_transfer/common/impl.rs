use super::*;

impl ResponseCode {
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
    T: Serialize + Default,
{
    pub fn default_success() -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::Success as i32)
            .set_message("Success".to_string())
            .set_data(None)
            .set_timestamp(Some(date()));
        instance
    }

    pub fn success(data: T) -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::Success as i32)
            .set_message("Success".to_string())
            .set_data(Some(data))
            .set_timestamp(Some(date()));
        instance
    }

    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::Success as i32)
            .set_message(message.into())
            .set_data(Some(data))
            .set_timestamp(Some(date()));
        instance
    }

    pub fn default_error() -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::InternalError as i32)
            .set_message("Internal server error".to_string())
            .set_data(None)
            .set_timestamp(Some(date()));
        instance
    }

    pub fn error(message: impl Into<String>) -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(ResponseCode::InternalError as i32)
            .set_message(message.into())
            .set_data(None)
            .set_timestamp(Some(date()));
        instance
    }

    pub fn error_with_code(code: ResponseCode, message: impl Into<String>) -> Self {
        let mut instance: ApiResponse<T> = Self::default();
        instance
            .set_code(code as i32)
            .set_message(message.into())
            .set_data(None)
            .set_timestamp(Some(date()));
        instance
    }

    pub fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }
}

impl ApiResponse<()> {
    pub fn success_without_data(message: impl Into<String>) -> Self {
        let mut instance: ApiResponse<()> = Self::default();
        instance
            .set_code(ResponseCode::Success as i32)
            .set_message(message.into())
            .set_data(None)
            .set_timestamp(Some(date()));
        instance
    }
}
