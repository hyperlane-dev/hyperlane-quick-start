use super::*;

pub static BROADCAST_CHANNEL: OnceLock<Broadcast<Vec<u8>>> = OnceLock::new();
