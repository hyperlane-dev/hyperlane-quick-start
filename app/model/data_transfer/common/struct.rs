use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            code: ResponseCode::Success as i32,
            message: "Success".to_string(),
            data: Some(data),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            code: ResponseCode::Success as i32,
            message: message.into(),
            data: Some(data),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            code: ResponseCode::InternalError as i32,
            message: message.into(),
            data: None,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
    pub fn error_with_code(code: ResponseCode, message: impl Into<String>) -> Self {
        Self {
            code: code as i32,
            message: message.into(),
            data: None,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
    pub fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }
}

impl ApiResponse<()> {
    pub fn success_without_data(message: impl Into<String>) -> Self {
        Self {
            code: ResponseCode::Success as i32,
            message: message.into(),
            data: None,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
