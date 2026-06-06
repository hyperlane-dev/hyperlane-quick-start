use super::*;

/// Standard API response structure with status code, message, data payload, and timestamp.
#[skip_serializing_none]
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: Clone + Default + Serialize,
{
    /// The response status code.
    #[get(type(copy))]
    pub(super) code: i32,
    /// The response message describing the result.
    #[set(type(AsRef<str>))]
    pub(super) message: String,
    /// The optional data payload of the response.
    pub(super) data: Option<T>,
    /// The optional timestamp of the response in milliseconds since epoch.
    #[get(type(copy))]
    pub(super) timestamp: Option<i64>,
}
