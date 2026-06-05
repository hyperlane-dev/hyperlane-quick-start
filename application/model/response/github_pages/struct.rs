use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesResourceResponse {
    pub(super) owner: String,
    pub(super) repository: String,
    pub(super) resources: Vec<GithubPagesResource>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesListResponse {
    pub(super) pages: Vec<GithubPagesInfo>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesInfo {
    #[get(type(copy))]
    pub(super) id: i32,
    pub(super) owner: String,
    pub(super) repository: String,
    pub(super) base_url: String,
    pub(super) last_synced_at: String,
    #[get(type(copy))]
    pub(super) resource_count: usize,
}
