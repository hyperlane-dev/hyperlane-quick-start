use super::*;

pub static BROADCAST_CHANNEL: OnceLock<Broadcast<ResponseBody>> = OnceLock::new();
