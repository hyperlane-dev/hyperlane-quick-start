use super::*;

/// Implementation of `EuvPlaygroundBootstrap` for `BootstrapAsyncInit`.
///
/// Creates the build topic + consumer group, attaches a worker listener
/// that processes one build task per published payload, and spawns a
/// periodic GC coroutine that evicts finished jobs past
/// [`BUILD_JOB_TTL_MS`].
impl BootstrapAsyncInit for EuvPlaygroundBootstrap {
    /// Creates the build topic and worker consumer group and spawns the worker
    /// listener together with the periodic GC coroutine.
    #[instrument_trace]
    async fn init() -> Self {
        let broker: &MessageQueueBroker = get_message_queue_broker();
        let topic: &str = topic_euv_playground_build();
        let consumer_group: &str = consumer_group_build_worker();
        if let Err(error) = broker
            .create_topic_with_capacity(topic, build_topic_capacity())
            .await
        {
            error!("{ERROR_CREATE_TOPIC_PREFIX} '{topic}' {error}");
        }
        if let Err(error) = broker.create_consumer_group(topic, consumer_group).await {
            error!("{ERROR_CREATE_CONSUMER_GROUP_PREFIX} '{consumer_group}' {error}");
        }
        listen_consumer_group(topic, consumer_group, |payload: MessagePayload| {
            spawn(EuvPlaygroundService::run_build_for_job(payload));
        });
        spawn(async move {
            loop {
                EuvPlaygroundService::purge_expired_jobs().await;
                tokio::time::sleep(Duration::from_millis(build_job_gc_interval_ms())).await;
            }
        });
        info!("Euv playground build pipeline initialized on topic '{topic}'");
        Self
    }
}
