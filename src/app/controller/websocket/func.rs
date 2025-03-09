use crate::*;

pub async fn handle(controller_data: ControllerData) {
    let request_body: Vec<u8> = controller_data.get_request_body().await;
    let _ = controller_data.send_response_body(request_body).await;
}
