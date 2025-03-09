use crate::*;
use std::borrow::Cow;

pub async fn log(controller_data: ControllerData) {
    let request: Request = controller_data.get_request().await;
    let request_method: &Methods = request.get_method();
    let request_host: &RequestHost = request.get_host();
    let request_version: &RequestVersion = request.get_version();
    let request_path: &RequestPath = request.get_path();
    let request_querys: &RequestQuerys = request.get_querys();
    let request_headers: &ResponseHeaders = request.get_headers();
    let request_body: Cow<'_, str> = String::from_utf8_lossy(request.get_body());
    let response: Response = controller_data.get_response().await;
    let response_version: &ResponseVersion = response.get_version();
    let response_headers: &ResponseHeaders = response.get_headers();
    let response_body: Cow<'_, str> = String::from_utf8_lossy(response.get_body());
    let status_code: &ResponseStatusCode = response.get_status_code();
    let reason_phrase: &ResponseReasonPhrase = response.get_reason_phrase();

    controller_data
        .log_info(format!("Request method => {}", request_method), log_handler)
        .await
        .log_info(format!("Request host => {}", request_host), log_handler)
        .await
        .log_info(
            format!("Request version => {}", request_version),
            log_handler,
        )
        .await
        .log_info(format!("Request path => {}", request_path), log_handler)
        .await
        .log_info(
            format!("Request querys => {:?}", request_querys),
            log_handler,
        )
        .await
        .log_info(
            format!("Request headers => {:?}", request_headers),
            log_handler,
        )
        .await
        .log_info(format!("Request body => {}", request_body), log_handler)
        .await
        .log_info(
            format!("Response version => {}", response_version),
            log_handler,
        )
        .await
        .log_info(
            format!("Response status code => {} {}", status_code, reason_phrase),
            log_handler,
        )
        .await
        .log_info(
            format!("Response headers => {:?}", response_headers),
            log_handler,
        )
        .await
        .log_info(format!("Response body => {}", response_body), log_handler)
        .await;
}
