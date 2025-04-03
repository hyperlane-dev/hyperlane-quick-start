use crate::*;

pub async fn cross(ctx: Context) {
    let _ = ctx
        .get_stream()
        .await
        .unwrap()
        .get_write_lock()
        .await
        .set_nodelay(true);
    ctx.set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, ANY)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_METHODS, ALL_METHODS)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_HEADERS, ANY)
        .await;
}
