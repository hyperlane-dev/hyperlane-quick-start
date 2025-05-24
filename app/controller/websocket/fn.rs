use super::*;

pub async fn handle(ctx: Context) {
    let broadcast: Broadcast<ResponseBody> = ensure_broadcast_channel();
    let mut receiver: BroadcastReceiver<Vec<u8>> = broadcast.subscribe();
    loop {
        tokio::select! {
            request_res = ctx.websocket_request_from_stream(10000) => {
                if request_res.is_err() {
                    break;
                }
                let request = request_res.unwrap_or_default();
                let body: RequestBody = request.get_body().clone();
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
