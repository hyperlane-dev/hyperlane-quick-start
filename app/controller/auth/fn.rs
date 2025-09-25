use super::*;

#[route("/auth/login")]
#[prologue_hooks(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn login(ctx: Context) {
    let response = handle_login(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/logout")]
#[prologue_hooks(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn logout(ctx: Context) {
    let response = handle_logout(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/session")]
#[prologue_hooks(
    get,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn session_info(ctx: Context) {
    let response = handle_session_info(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/register")]
#[prologue_hooks(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn register(ctx: Context) {
    let response = handle_register(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/change-password")]
#[prologue_hooks(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn change_password(ctx: Context) {
    let response = handle_change_password(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/get-profile")]
#[prologue_hooks(
    get,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn get_profile(ctx: Context) {
    let response = handle_get_profile(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/update-profile")]
#[prologue_hooks(
    put,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn update_profile(ctx: Context) {
    let response = handle_update_profile(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/check-username")]
#[prologue_hooks(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn check_username(ctx: Context) {
    let response = handle_check_username(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/check-email")]
#[prologue_hooks(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn check_email(ctx: Context) {
    let response = handle_check_email(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}

#[route("/auth/validate-session")]
#[prologue_hooks(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn validate_session(ctx: Context) {
    let response = handle_validate_session(&ctx).await;
    let response_json = serde_json::to_vec(&response).unwrap_or_default();
    ctx.set_response_body(&response_json).await;
}
