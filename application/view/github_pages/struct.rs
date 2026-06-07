use super::*;

/// github pages proxy root route.
#[route("/github/pages/{owner}/{repository}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GithubPagesProxyRootRoute;

/// github pages proxy route.
#[route("/github/pages/{owner}/{repository}/{path:.*}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GithubPagesProxyRoute;
