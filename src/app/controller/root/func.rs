use crate::*;

pub async fn handle(controller_data: ControllerData) {
    let _ = controller_data
        .set_response_body("Hello hyperlane => /")
        .await;
}
