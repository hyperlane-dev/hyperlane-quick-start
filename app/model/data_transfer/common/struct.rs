use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Data)]
pub struct ApiResponse<T>
where
    T: Serialize + Default,
{
    code: i32,
    message: String,
    data: Option<T>,
    timestamp: Option<String>,
}
