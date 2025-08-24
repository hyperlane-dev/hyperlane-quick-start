use super::*;

#[route("/api/users/online")]
#[utoipa::path(
    get,
    path = "/api/users/online",
    responses(
        (status = 200, description = "Get online users list", body = UserListResponse)
    )
)]
#[prologue_hooks[
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
]]
pub async fn online_users(ctx: Context) {
    let user_list: UserListResponse = get_online_users_list();
    let response_json: String = serde_json::to_string(&user_list).unwrap_or_default();
    ctx.set_response_body(response_json).await;
}
