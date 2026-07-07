use super::*;

/// Implementation of `BootstrapAsyncInit` for `MessageQueueBootstrap`.
///
/// Initializes the global message queue broker, creates configured topics and
/// consumer groups, and spawns a dedicated listener thread for each consumer
/// so that messages start being processed immediately upon server startup.
impl BootstrapAsyncInit for MessageQueueBootstrap {
    #[instrument_trace]
    async fn init() -> Self {
        let broker: &MessageQueueBroker = get_message_queue_broker();
        let topic_count: usize = broker.topic_count().await;
        let group_count: usize = broker.consumer_group_count().await;
        info!(
            "Message queue plugin initialized with {topic_count} topic(s) and {group_count} consumer group(s)"
        );
        Self
    }
}
