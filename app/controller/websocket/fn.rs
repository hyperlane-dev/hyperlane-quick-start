use super::*;
use hyperlane::tokio::sync::broadcast::{Receiver, Sender, channel};
use std::sync::OnceLock;

type Message = Vec<u8>;
static BROADCAST_CHANNEL: OnceLock<Sender<Message>> = OnceLock::new();

fn ensure_broadcast_channel() -> Sender<Message> {
    BROADCAST_CHANNEL
        .get_or_init(|| {
            let (sender, _) = channel(1);
            sender
        })
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
    let sender: Sender<Vec<u8>> = ensure_broadcast_channel();
    let ctx: Context = Context::from_stream_request_log(&stream, &first_request, &log);
    let mut receiver: Receiver<Vec<u8>> = sender.subscribe();
    loop {
        tokio::select! {
            request_res = Request::websocket_request_from_stream(&stream, 10000) => {
                if request_res.is_err() {
                    break;
                }
                let request = request_res.unwrap_or_default();
                let body: RequestBody = request.get_body().clone();
                first_request.set_body(body.clone());
                if sender.send(body).is_err() {
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
