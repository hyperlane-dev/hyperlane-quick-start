use crate::*;

pub async fn root(controller_data: ControllerData) {
    let _ = controller_data
        .send_response(200, "Hello hyperlane => /")
        .await;
}
