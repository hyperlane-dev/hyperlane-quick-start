use super::*;

pub(super) static SHUTDOWN: OnceLock<ArcFnPinBoxFutureSend<()>> = OnceLock::new();
