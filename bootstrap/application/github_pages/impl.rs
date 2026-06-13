use super::*;

/// Implementation of `GithubPagesBootstrap` for `BootstrapAsyncInit`.
impl BootstrapAsyncInit for GithubPagesBootstrap {
    #[instrument_trace]
    async fn init() -> Self {
        let handles: Vec<JoinHandle<()>> = SYNC_REPOSITORIES
            .iter()
            .map(|&(owner, repository)| {
                let owner_owned: String = owner.to_string();
                let repository_owned: String = repository.to_string();
                spawn(async move {
                    match GithubPagesService::sync_github_pages(&owner_owned, &repository_owned).await
                    {
                        Ok(()) => {
                            info!("Synced GitHub Pages {owner_owned}/{repository_owned}");
                        }
                        Err(error) => {
                            error!(
                                "Failed to sync GitHub Pages {owner_owned}/{repository_owned} {error}"
                            );
                        }
                    }
                })
            })
            .collect();
        join_all(handles).await;
        Self
    }
}
