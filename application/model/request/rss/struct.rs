use super::*;

/// Represents a request to generate an RSS feed with optional filters.
#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema, Data)]
pub struct RssFeedRequest {
    /// The limit.
    #[get(type(copy))]
    pub limit: Option<usize>,
    /// The offset.
    #[get(type(copy))]
    pub offset: Option<usize>,
    /// The timezone.
    pub timezone: Option<Timezone>,
}
