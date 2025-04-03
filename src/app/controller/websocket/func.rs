use crate::*;
use hyperlane::{
    once_cell::sync::Lazy,
    tokio::{
        spawn,
        sync::{RwLock, broadcast::*},
    },
};
use std::{net::SocketAddr, sync::Arc};

static MAP: Lazy<DashMap<SocketAddr, bool>> = Lazy::new(DashMap::new);
static SENDER: Lazy<Arc<RwLock<Sender<Vec<u8>>>>> = Lazy::new(|| {
    let (send, _) = channel(100);
    arc_rwlock(send)
});

pub async fn handle(ctx: Context) {
    let addr: SocketAddr = ctx.get_socket_addr_or_default().await;
    let sender: Sender<Vec<u8>> = SENDER.read().await.clone();
    if MAP.get(&addr).is_none() {
        let ctx_clone: Context = ctx.clone();
        let mut receiver: Receiver<Vec<u8>> = sender.subscribe();
        spawn(async move {
            println_warning!("subscribe");
            while let Ok(data) = receiver.recv().await {
                println_warning!("Received data from", addr);
                let _ = ctx_clone.send_response_body(data).await;
            }
            println_warning!("end");
        });
    }
    MAP.insert(addr, true);
    let request_body: Vec<u8> = ctx.get_request_body().await;
    let _ = sender.send(request_body);
}
