use super::*;

/// Implementation of `GithubPagesBootstrap` for `BootstrapAsyncInit`.
///
/// Creates the `github_pages_sync` topic and `sync_worker` consumer group,
/// starts a background consumer that processes sync tasks, and publishes
/// initial sync messages for all repositories listed in `SYNC_REPOSITORIES`.
impl BootstrapAsyncInit for GithubPagesBootstrap {
    #[instrument_trace]
    async fn init() -> Self {
        let broker: &MessageQueueBroker = get_message_queue_broker();
        if let Err(error) = broker.create_topic(TOPIC_GITHUB_PAGES_SYNC).await {
            error!("Failed to create topic '{TOPIC_GITHUB_PAGES_SYNC}' {error}");
        }
        if let Err(error) = broker
            .create_consumer_group(TOPIC_GITHUB_PAGES_SYNC, CONSUMER_GROUP_SYNC_WORKER)
            .await
        {
            error!("Failed to create consumer group '{CONSUMER_GROUP_SYNC_WORKER}' {error}");
        }
        listen_consumer_group(
            TOPIC_GITHUB_PAGES_SYNC,
            CONSUMER_GROUP_SYNC_WORKER,
            |payload: MessagePayload| {
                let message: String = match String::from_utf8(payload) {
                    Ok(msg) => msg,
                    Err(error) => {
                        error!("Failed to parse sync task payload {error}");
                        return;
                    }
                };
                let parts: Vec<&str> = message.split(SYNC_TASK_SEPARATOR).collect();
                let owner: &str = match parts.first() {
                    Some(owner_part) => owner_part,
                    None => {
                        error!("Invalid sync task payload: missing owner");
                        return;
                    }
                };
                let repository: &str = match parts.get(1) {
                    Some(repository_part) => repository_part,
                    None => {
                        error!("Invalid sync task payload: missing repository");
                        return;
                    }
                };
                let owner_owned: String = owner.to_string();
                let repository_owned: String = repository.to_string();
                spawn(async move {
                    match GithubPagesService::sync_github_pages(&owner_owned, &repository_owned)
                        .await
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
                });
            },
        );
        for &(owner, repository) in SYNC_REPOSITORIES {
            if let Err(error) = GithubPagesService::publish_sync_task(owner, repository).await {
                error!("Failed to publish initial sync task for {owner}/{repository} {error}");
            }
        }
        Self
    }
}
