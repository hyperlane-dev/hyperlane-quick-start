use super::*;

/// Global static storage for the message queue broker, initialized once on first access.
pub(crate) static MESSAGE_QUEUE_BROKER: OnceLock<MessageQueueBroker> = OnceLock::new();
