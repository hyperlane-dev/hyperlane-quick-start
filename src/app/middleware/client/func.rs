use crate::*;

pub async fn client(arc_lock_controller_data: ArcRwLockControllerData) {
    let socket_addr: String = arc_lock_controller_data
        .get_socket_addr()
        .await
        .unwrap_or_default();
    arc_lock_controller_data
        .set_response_header("SocketAddr", socket_addr)
        .await;
}
