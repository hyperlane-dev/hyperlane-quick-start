use super::*;

/// Implementation of `GithubPagesBootstrap` for `BootstrapAsyncInit`.
impl BootstrapAsyncInit for GithubPagesBootstrap {
    #[instrument_trace]
    async fn init() -> Self {
        GithubPagesService::start_sync_timer().await;
        Self
    }
}
