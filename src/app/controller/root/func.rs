use crate::*;

pub async fn handle(controller_data: ControllerData) {
    let _ = controller_data
        .send_response(200, "Hello hyperlane => /")
        .await;
}
