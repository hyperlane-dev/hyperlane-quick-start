use super::*;

/// Standard API response structure with status code, message, data payload, and timestamp.
#[skip_serializing_none]
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: Clone + Default + Serialize,
{
    /// The response status code.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) code: i32,
    /// The response message describing the result.
    #[set(pub(crate), type(AsRef<str>))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) message: String,
    /// The optional data payload of the response.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) data: Option<T>,
    /// The optional timestamp of the response in milliseconds since epoch.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timestamp: Option<i64>,
}
