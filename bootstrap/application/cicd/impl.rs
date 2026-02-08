use super::*;

impl BootstrapAsyncInit for CicdBootstrap {
    #[instrument_trace]
    async fn init() -> Self {
        match CicdService::recover_interrupted_runs().await {
            Ok(count) => {
                if count > 0 {
                    warn!("Recovered {count} interrupted CICD runs due to server restart");
                } else {
                    info!("No interrupted CICD runs found");
                }
            }
            Err(error) => {
                error!("Failed to recover interrupted CICD runs {error}");
            }
        }
        Self
    }
}
