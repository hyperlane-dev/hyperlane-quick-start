/// Delivery guarantee level for message publishing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryGuarantee {
    /// At-most-once: fire-and-forget, no acknowledgment required.
    AtMostOnce,
    /// At-least-once: the message is retried until at least one consumer acknowledges it.
    AtLeastOnce,
}

/// Lifecycle state of a topic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopicState {
    /// The topic is active and accepting messages.
    Active,
    /// The topic is paused; published messages are buffered but not delivered.
    Paused,
    /// The topic has been closed and can no longer accept messages.
    Closed,
}
