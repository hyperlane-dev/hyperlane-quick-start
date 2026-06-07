use super::*;

/// sync github pages request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct SyncGithubPagesRequest {
    /// The repository.
    pub repository: String,
}
