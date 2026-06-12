use super::*;

/// Placeholder module for GitHub Pages request models.
///
/// The simplified GitHub Pages module no longer requires dedicated request types
/// since all operations use route parameters instead of request bodies.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GithubPagesRequestModel;
