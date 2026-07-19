use super::*;

/// Application-level model representing a short link record with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShortlinkRecord {
    /// The unique identifier used as the short link token.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The original target URL that the short link redirects to.
    pub(super) url: String,
    /// The string-formatted timestamp when the short link was created.
    pub(super) created_at: String,
}
