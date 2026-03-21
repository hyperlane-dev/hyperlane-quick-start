use super::*;

#[skip_serializing_none]
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: Clone + Default + Serialize,
{
    #[get(type(copy))]
    pub(super) code: i32,
    pub(super) message: String,
    pub(super) data: Option<T>,
    pub(super) timestamp: Option<String>,
}
