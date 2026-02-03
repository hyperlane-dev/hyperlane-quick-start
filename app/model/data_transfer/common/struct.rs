use super::*;

#[skip_serializing_none]
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: Clone + Default + Serialize,
{
    #[get(type(copy), pub(crate))]
    pub(super) code: i32,
    #[get(pub(crate))]
    pub(super) message: String,
    #[get(pub(crate))]
    pub(super) data: Option<T>,
    #[get(pub(crate))]
    pub(super) timestamp: Option<String>,
}
