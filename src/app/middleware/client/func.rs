use crate::*;

pub async fn client(arc_lock_controller_data: ArcRwLockControllerData) {
    let socket_addr: String = arc_lock_controller_data
        .get_socket_addr()
        .await
        .unwrap_or_default();
    let mut controller_data: RwLockWriteControllerData =
        arc_lock_controller_data.get_write_lock().await;
    let response: &mut Response = controller_data.get_mut_response();
    response.set_header("SocketAddr", socket_addr);
}
