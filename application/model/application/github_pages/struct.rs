use super::*;

/// github pages resource.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesResource {
    /// The owner.
    pub(super) owner: String,
    /// The repository.
    pub(super) repository: String,
    /// The path.
    pub(super) path: String,
    /// The content type.
    pub(super) content_type: String,
    /// The file size.
    #[get(type(copy))]
    pub(super) file_size: u64,
    /// The local path.
    pub(super) local_path: String,
    /// The url.
    pub(super) url: String,
}
