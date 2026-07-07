use super::*;

/// Bootstrap handler for initializing the message queue plugin.
///
/// On initialization it creates the global broker, registers any topics and
/// consumer groups declared in the environment configuration, and spawns
/// a dedicated thread for each registered consumer so that messages are
/// processed concurrently from startup.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct MessageQueueBootstrap;
