use super::*;

/// Serializes the request context to a JSON string, replacing the body with its length for logging purposes.
///
/// # Arguments
///
/// - `&Context`: The request context to serialize.
///
/// # Returns
///
/// - `String`: The JSON string representation of the request with body length instead of full body.
#[instrument_trace]
pub async fn get_request_json(ctx: &Context) -> String {
    let mut request: Request = ctx.get_request().clone();
    request.set_body(request.get_body().len().to_string().into_bytes());
    serde_json::to_string(&request).unwrap_or(request.to_string())
}

/// Serializes the response context to a JSON string, replacing the body with its length for logging purposes.
///
/// # Arguments
///
/// - `&Context`: The response context to serialize.
///
/// # Returns
///
/// - `String`: The JSON string representation of the response with body length instead of full body.
#[instrument_trace]
pub async fn get_response_json(ctx: &Context) -> String {
    let mut response: Response = ctx.get_response().clone();
    response.set_body(response.get_body().len().to_string().into_bytes());
    serde_json::to_string(&response).unwrap_or(response.to_string())
}
