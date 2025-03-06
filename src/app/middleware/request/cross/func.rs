use crate::*;

pub async fn cross(controller_data: ControllerData) {
    controller_data
        .set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, ANY)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_METHODS, GET_POST_OPTIONS)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_HEADERS, ANY)
        .await
        .set_response_header(
            CONTENT_TYPE,
            format!("{}; {}", APPLICATION_JSON, CHARSET_UTF_8),
        )
        .await;
}
