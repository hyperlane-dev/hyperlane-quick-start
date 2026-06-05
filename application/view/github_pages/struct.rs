use super::*;

#[route("/github/pages/{owner}/{repository}/{path:.*}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GithubPagesProxyRoute;
