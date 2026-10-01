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
        if let Err(error) = broker
            .create_topic_with_capacity(TOPIC_EUV_PLAYGROUND_BUILD, BUILD_TOPIC_CAPACITY)
            .await
        {
            error!("{ERROR_CREATE_TOPIC_PREFIX} '{TOPIC_EUV_PLAYGROUND_BUILD}' {error}");
        }
        if let Err(error) = broker
            .create_consumer_group(TOPIC_EUV_PLAYGROUND_BUILD, CONSUMER_GROUP_BUILD_WORKER)
            .await
        {
            error!("{ERROR_CREATE_CONSUMER_GROUP_PREFIX} '{CONSUMER_GROUP_BUILD_WORKER}' {error}");
        }
        listen_consumer_group(
            TOPIC_EUV_PLAYGROUND_BUILD,
            CONSUMER_GROUP_BUILD_WORKER,
            |payload: MessagePayload| {
                spawn(EuvPlaygroundService::run_build_for_job(payload));
            },
        );
        spawn(async move {
            loop {
                EuvPlaygroundService::purge_expired_jobs().await;
                tokio::time::sleep(Duration::from_millis(BUILD_JOB_GC_INTERVAL_MS)).await;
            }
        });
        info!("Euv playground build pipeline initialized on topic '{TOPIC_EUV_PLAYGROUND_BUILD}'");
        Self
    }
}
