use super::*;

pub(super) static SHUTDOWN: OnceLock<ArcPinBoxFutureSend> = OnceLock::new();
