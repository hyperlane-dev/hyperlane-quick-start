use crate::*;

pub async fn log(controller_data: ControllerData) {
    let request: String = controller_data.get_request().await.get_string();
    let response: String = controller_data.get_response().await.get_string();
    controller_data
        .log_info(request, log_handler)
        .await
        .log_info(response, log_handler)
        .await;
}
