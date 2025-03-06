use crate::*;

pub async fn log(controller_data: ControllerData) {
    let request: String = controller_data.get_request().await.to_string();
    let response: String = controller_data.get_response().await.to_string();
    controller_data
        .log_info(format!("Request => {}", request), log_handler)
        .await
        .log_info(format!("Response => {}", response), log_handler)
        .await;
}
