use super::*;

/// sync github pages route.
#[route("/api/github/pages/sync")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct SyncGithubPagesRoute;

/// list github pages route.
#[route("/api/github/pages/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListGithubPagesRoute;

/// get github pages resources route.
#[route("/api/github/pages/{owner}/{repository}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetGithubPagesResourcesRoute;

/// delete github pages route.
#[route("/api/github/pages/delete/{owner}/{repository}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DeleteGithubPagesRoute;
