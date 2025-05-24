use super::*;

pub fn ensure_broadcast_channel() -> Broadcast<ResponseBody> {
    BROADCAST_CHANNEL
        .get_or_init(|| Broadcast::default())
        .clone()
}
