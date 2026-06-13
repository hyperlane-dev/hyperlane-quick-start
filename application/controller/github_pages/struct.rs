use super::*;

/// list github pages route.
#[route("/api/github/pages/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListGithubPagesRoute;

/// sync github pages route.
#[route("/api/github/pages/sync/{owner}/{repository}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct SyncGithubPagesRoute;
