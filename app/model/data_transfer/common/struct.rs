use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Data)]
pub struct ApiResponse<T>
where
    T: Serialize + Default,
{
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize + Default,
{
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
