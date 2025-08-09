use super::*;

pub fn set_shutdown(shutdown: ArcPinBoxFutureSend) {
    if SHUTDOWN.get().is_some() {
        return;
    }
    let _ = SHUTDOWN.set(shutdown);
}

pub fn shutdown() -> ArcPinBoxFutureSend {
    SHUTDOWN
        .get_or_init(|| Arc::new(|| Box::pin(async {})))
        .clone()
}
