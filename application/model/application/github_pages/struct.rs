use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesResource {
    pub(super) owner: String,
    pub(super) repository: String,
    pub(super) path: String,
    pub(super) content_type: String,
    #[get(type(copy))]
    pub(super) file_size: u64,
    pub(super) local_path: String,
    pub(super) url: String,
}
