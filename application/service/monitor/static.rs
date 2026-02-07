use super::*;

pub(super) static SYSTEM: OnceLock<RwLock<System>> = OnceLock::new();
pub(super) static NETWORKS: OnceLock<RwLock<Networks>> = OnceLock::new();
