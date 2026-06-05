use super::*;

#[route("/api/github/pages/add")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AddGithubPagesRoute;

#[route("/api/github/pages/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListGithubPagesRoute;

#[route("/api/github/pages/{owner}/{repository}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetGithubPagesResourcesRoute;

#[route("/api/github/pages/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DeleteGithubPagesRoute;

#[route("/api/github/pages/sync")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct SyncGithubPagesRoute;
