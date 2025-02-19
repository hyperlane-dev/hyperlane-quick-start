use crate::*;

pub async fn client(arc_lock_controller_data: ArcRwLockControllerData) {
    let socket_addr: String = get_socket_addr(&arc_lock_controller_data)
        .await
        .unwrap_or_default();
    let mut controller_data: RwLockWriteControllerData =
        get_rw_lock_write_controller_data(&arc_lock_controller_data).await;
    let response: &mut Response = controller_data.get_mut_response();
    response.set_header("SocketAddr", socket_addr);
}
