use super::*;

/// github pages list response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesListResponse {
    /// The pages.
    pub(super) pages: Vec<GithubPagesInfo>,
}

/// Represents cached GitHub Pages information including owner, repository, and base URL.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesInfo {
    /// The owner.
    pub(super) owner: String,
    /// The repository.
    pub(super) repository: String,
    /// The base url.
    pub(super) base_url: String,
    /// The last synced at.
    pub(super) last_synced_at: String,
}
