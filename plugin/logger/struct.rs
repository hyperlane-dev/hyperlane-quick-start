use super::*;

/// Plugin for initializing and accessing the global logger.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct LoggerPlugin;

/// Custom logger implementation supporting both console and file logging.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct Logger;
