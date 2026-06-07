use super::*;

/// shortlink insert request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShortlinkInsertRequest {
    /// The url.
    pub(super) url: String,
}
