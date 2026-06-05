use super::*;

#[route("/github/pages/{owner}/{repository}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GithubPagesProxyRootRoute;

#[route("/github/pages/{owner}/{repository}/{path:.*}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GithubPagesProxyRoute;
