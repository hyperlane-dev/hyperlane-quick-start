use super::*;

#[skip_serializing_none]
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: Serialize + Default,
{
    code: i32,
    message: String,
    data: Option<T>,
    timestamp: Option<String>,
}
