use crate::*;

pub async fn root(controller_data: ControllerData) {
    let send_res: ResponseResult = controller_data
        .send_response(200, "hello hyperlane => /")
        .await;
    let request: Request = controller_data.get_request().await;
    controller_data
        .log_info(format!("Request result => {}", request), log_handler)
        .await
        .log_info(format!("Response result => {:?}", send_res), log_handler)
        .await;
}
