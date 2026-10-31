/// `tracing::error!` prefix when the build message queue topic cannot
/// be created at server startup. Caller does
/// `error!("{ERROR_CREATE_TOPIC_PREFIX} '{TOPIC_EUV_PLAYGROUND_BUILD}' {error}")`.
pub(crate) const ERROR_CREATE_TOPIC_PREFIX: &str = "Failed to create topic";

/// `tracing::error!` prefix when the build worker consumer group
/// cannot be created at server startup. Caller does
/// `error!("{ERROR_CREATE_CONSUMER_GROUP_PREFIX} '{CONSUMER_GROUP_BUILD_WORKER}' {error}")`.
pub(crate) const ERROR_CREATE_CONSUMER_GROUP_PREFIX: &str = "Failed to create consumer group";
