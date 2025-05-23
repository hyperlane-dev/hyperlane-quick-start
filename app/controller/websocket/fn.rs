use super::*;

static BROADCAST_CHANNEL: OnceLock<Broadcast<ResponseBody>> = OnceLock::new();

fn ensure_broadcast_channel() -> Broadcast<ResponseBody> {
    BROADCAST_CHANNEL
        .get_or_init(|| Broadcast::default())
        .clone()
}

pub async fn handle(ctx: Context) {
    if ctx.get_stream().await.is_none() {
        ctx.aborted().await;
        return;
    }
    let stream: ArcRwLockStream = ctx.get_stream().await.unwrap();
    let mut first_request: Request = ctx.get_request().await;
    let log: Log = ctx.get_log().await;
    let broadcast: Broadcast<ResponseBody> = ensure_broadcast_channel();
    let ctx: Context = Context::from_stream_request_log(&stream, &first_request, &log);
    let mut receiver: BroadcastReceiver<Vec<u8>> = broadcast.subscribe();
    loop {
        tokio::select! {
            request_res = Request::websocket_request_from_stream(&stream, 10000) => {
                if request_res.is_err() {
                    break;
                }
                let request = request_res.unwrap_or_default();
                let body: RequestBody = request.get_body().clone();
                first_request.set_body(body.clone());
                if broadcast.send(body).is_err() {
                    break;
                }
            },
            msg_res = receiver.recv() => {
                if let Ok(msg) = msg_res {
                    if ctx.send_response_body(msg).await.is_err() || ctx.flush().await.is_err() {
                        break;
                    }
                }
           }
        }
    }
}
