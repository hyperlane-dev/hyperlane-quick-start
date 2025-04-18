use super::*;

pub async fn cross(ctx: Context) {
    ctx.set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, ANY)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_METHODS, ALL_METHODS)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_HEADERS, ANY)
        .await;
}
