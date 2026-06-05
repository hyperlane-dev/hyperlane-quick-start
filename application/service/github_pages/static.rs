use super::*;

pub(super) static GITHUB_PAGES_RESOURCES: OnceLock<
    RwLock<HashMap<String, Vec<GithubPagesResource>>>,
> = OnceLock::new();
