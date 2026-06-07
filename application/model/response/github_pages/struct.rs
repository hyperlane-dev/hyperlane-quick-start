use super::*;

/// github pages resource response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesResourceResponse {
    /// The owner.
    pub(super) owner: String,
    /// The repository.
    pub(super) repository: String,
    /// The resources.
    pub(super) resources: Vec<GithubPagesResource>,
}

/// github pages list response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesListResponse {
    /// The pages.
    pub(super) pages: Vec<GithubPagesInfo>,
}

/// Represents cached GitHub Pages information including owner, repository, and resource list.
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
    /// The resource count.
    #[get(type(copy))]
    pub(super) resource_count: usize,
}
