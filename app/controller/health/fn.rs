use super::*;
use hyperlane::*;
use serde_json;

#[route("/health")]
#[prologue_hooks(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn health_check(ctx: Context) {
    let response = serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}
