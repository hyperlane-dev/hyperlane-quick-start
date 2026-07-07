use super::*;

/// Returns a static reference to the global message queue broker, initializing it on first access.
///
/// # Returns
///
/// - `&'static MessageQueueBroker`: The static reference to the global broker.
#[instrument_trace]
pub fn get_message_queue_broker() -> &'static MessageQueueBroker {
    MESSAGE_QUEUE_BROKER.get_or_init(MessageQueueBroker::new)
}

/// Starts a background listener task that reads messages from a topic-level receiver
/// and invokes the provided handler closure for each message.
///
/// The listener runs on its own tokio task and will continue until the topic is
/// closed or the receiver encounters a lagged error it cannot recover from.
///
/// # Arguments
///
/// - `&str`: The topic name to listen to.
/// - `F`: The async handler closure invoked for each received message payload.
///
/// # Returns
///
/// - `JoinHandle<()>`: The join handle for the spawned listener task.
///
/// # Panics
///
/// Panics if the topic does not exist when the listener starts.
pub fn listen_topic<F>(topic_name: &str, handler: F) -> JoinHandle<()>
where
    F: Fn(MessagePayload) + Send + 'static,
{
    let topic_name_owned: String = topic_name.to_string();
    spawn(async move {
        let broker: &MessageQueueBroker = get_message_queue_broker();
        let mut receiver: TopicReceiver = match broker.subscribe(&topic_name_owned).await {
            Ok(rx) => rx,
            Err(error) => {
                error!("Failed to subscribe to topic '{topic_name_owned}' {error}");
                return;
            }
        };
        loop {
            match receiver.recv().await {
                Ok(payload) => handler(payload),
                Err(RecvError::Lagged(count)) => {
                    warn!("Topic '{topic_name_owned}' receiver lagged, skipped {count} messages");
                }
                Err(RecvError::Closed) => {
                    info!("Topic '{topic_name_owned}' closed, listener exiting");
                    break;
                }
            }
        }
    })
}

/// Starts a background listener task that reads messages from a consumer-group
/// receiver and invokes the provided handler closure for each message.
///
/// Each call creates an independent consumer within the group, so all consumers
/// in the same group each receive a copy of every message (pub/sub semantics).
///
/// # Arguments
///
/// - `&str`: The topic name the group is bound to.
/// - `&str`: The consumer group name.
/// - `F`: The async handler closure invoked for each received message payload.
///
/// # Returns
///
/// - `JoinHandle<()>`: The join handle for the spawned listener task.
///
/// # Panics
///
/// Panics if the topic or consumer group does not exist when the listener starts.
pub fn listen_consumer_group<F>(topic_name: &str, group_name: &str, handler: F) -> JoinHandle<()>
where
    F: Fn(MessagePayload) + Send + 'static,
{
    let topic_name_owned: String = topic_name.to_string();
    let group_name_owned: String = group_name.to_string();
    spawn(async move {
        let broker: &MessageQueueBroker = get_message_queue_broker();
        let mut receiver: TopicReceiver = match broker
            .subscribe_group(&topic_name_owned, &group_name_owned)
            .await
        {
            Ok(rx) => rx,
            Err(error) => {
                error!(
                    "Failed to subscribe to group '{group_name_owned}' on topic '{topic_name_owned}' {error}"
                );
                return;
            }
        };
        loop {
            match receiver.recv().await {
                Ok(payload) => handler(payload),
                Err(RecvError::Lagged(count)) => {
                    warn!(
                        "Group '{group_name_owned}' on topic '{topic_name_owned}' receiver lagged, skipped {count} messages"
                    );
                }
                Err(RecvError::Closed) => {
                    info!(
                        "Group '{group_name_owned}' on topic '{topic_name_owned}' closed, listener exiting"
                    );
                    break;
                }
            }
        }
    })
}
