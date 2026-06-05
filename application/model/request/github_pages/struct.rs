use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct AddGithubPagesRequest {
    pub(super) owner: String,
    pub(super) repository: String,
}
