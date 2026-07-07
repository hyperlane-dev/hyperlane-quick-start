use super::*;

/// Type alias for the message payload — raw bytes allowing any serializable content.
pub type MessagePayload = Vec<u8>;

/// Type alias for a broadcast sender carrying message payloads.
pub type TopicSender = Sender<MessagePayload>;

/// Type alias for a broadcast receiver carrying message payloads.
pub type TopicReceiver = Receiver<MessagePayload>;

/// Type alias for the global topic registry, mapping topic names to topic instances.
pub type TopicRegistry = HashMap<String, Topic>;

/// Type alias for the consumer group registry, mapping composite keys to group instances.
pub type ConsumerGroupRegistry = HashMap<String, ConsumerGroup>;

/// Type alias for the set of registered message handlers.
pub type MessageHandlerRegistry = Vec<MessageHandler>;
