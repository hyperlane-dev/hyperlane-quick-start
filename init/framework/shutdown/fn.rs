use super::*;

pub fn set_shutdown(shutdown: ArcFnPinBoxFutureSend<()>) {
    let _ = SHUTDOWN.set(shutdown);
}

pub fn shutdown() -> ArcFnPinBoxFutureSend<()> {
    SHUTDOWN
        .get_or_init(|| Arc::new(|| Box::pin(async {})))
        .clone()
}
