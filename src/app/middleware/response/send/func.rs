use crate::*;

pub async fn send(controller_data: ControllerData) {
    let _ = controller_data.send().await;
}
