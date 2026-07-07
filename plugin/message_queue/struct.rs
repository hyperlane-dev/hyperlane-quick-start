use super::*;

/// Plugin entry point for the message queue system, managing topic lifecycle and message routing.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct MessageQueuePlugin;

/// A named topic within the message queue, holding its own broadcast channel and metadata.
#[derive(Clone, Data, Debug)]
pub struct Topic {
    /// The unique name identifying this topic.
    pub(super) name: String,
    /// The broadcast sender used to publish messages to all subscribers of this topic.
    pub(super) sender: TopicSender,
    /// The current lifecycle state of the topic.
    pub(super) state: TopicState,
}

/// A consumer group bound to a specific topic, distributing messages among its members.
#[derive(Clone, Data, Debug)]
pub struct ConsumerGroup {
    /// The name of the consumer group.
    pub(super) name: String,
    /// The name of the topic this consumer group is bound to.
    pub(super) topic_name: String,
    /// The broadcast sender used to fan out messages to group members.
    pub(super) sender: TopicSender,
}

/// A registered message handler that processes messages from a topic or consumer group.
#[derive(Clone, Data, Debug)]
pub struct MessageHandler {
    /// The name of the handler, used for identification and logging.
    pub(super) name: String,
    /// The topic name this handler is subscribed to.
    pub(super) topic_name: String,
    /// The optional consumer group name this handler belongs to.
    pub(super) group_name: Option<String>,
}

/// Central broker managing topics, consumer groups, and message dispatching.
///
/// Provides a unified API for creating topics, subscribing consumers, publishing
/// messages, and starting background listener tasks that feed topic messages into
/// consumer-group broadcast channels.
#[derive(Clone, Data, Debug)]
pub struct MessageQueueBroker {
    /// The registry of all active topics, keyed by topic name.
    pub(super) topics: ArcRwLock<TopicRegistry>,
    /// The registry of all consumer groups, keyed by `"{topic_name}::{group_name}"`.
    pub(super) consumer_groups: ArcRwLock<ConsumerGroupRegistry>,
}
