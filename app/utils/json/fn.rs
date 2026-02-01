use super::*;

#[instrument_trace]
pub async fn get_request_json(ctx: &Context) -> String {
    let mut request: Request = ctx.get_request().await;
    request.set_body(request.get_body().len().to_string().into_bytes());
    serde_json::to_string(&request).unwrap_or(request.to_string())
}

#[instrument_trace]
pub async fn get_response_json(ctx: &Context) -> String {
    let mut response: Response = ctx.get_response().await;
    response.set_body(response.get_body().len().to_string().into_bytes());
    serde_json::to_string(&response).unwrap_or(response.to_string())
}
