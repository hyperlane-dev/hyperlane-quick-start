/// Default channel capacity for the internal mpsc sender when creating a topic.
pub const DEFAULT_TOPIC_CAPACITY: usize = 256;

/// Error message returned when a topic with the given name already exists.
pub const ERROR_TOPIC_ALREADY_EXISTS: &str = "Topic already exists";

/// Error message returned when a topic with the given name does not exist.
pub const ERROR_TOPIC_NOT_FOUND: &str = "Topic not found";

/// Error message returned when a consumer group with the given name already exists for a topic.
pub const ERROR_CONSUMER_GROUP_ALREADY_EXISTS: &str = "Consumer group already exists";

/// Error message returned when a consumer group with the given name does not exist for a topic.
pub const ERROR_CONSUMER_GROUP_NOT_FOUND: &str = "Consumer group not found";
