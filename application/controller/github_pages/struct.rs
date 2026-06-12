use super::*;

/// list github pages route.
#[route("/api/github/pages/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListGithubPagesRoute;

/// delete github pages route.
#[route("/api/github/pages/delete/{owner}/{repository}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DeleteGithubPagesRoute;
