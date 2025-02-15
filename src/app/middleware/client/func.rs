use crate::*;

pub fn client(arc_lock_controller_data: ArcRwLockControllerData) {
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
    let path: &String = controller_data.get_request().get_path();
    let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
    match stream.peer_addr() {
        Ok(client_host_port) => {
            println_success!(client_host_port, " visit => ", path);
        }
        Err(err) => {
            println_error!(err, " visit => ", path);
        }
    }
}
