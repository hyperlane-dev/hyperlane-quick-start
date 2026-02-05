use super::*;

#[instrument_trace]
pub async fn init_cicd() {
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
}
