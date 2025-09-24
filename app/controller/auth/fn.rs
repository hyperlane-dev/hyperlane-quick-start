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

/// Handle login request
async fn handle_login(ctx: &Context) -> LoginResponse {
    // Parse request body
    let request_body = ctx.get_request_body().await;
    let login_request: Result<LoginRequest, _> = serde_json::from_slice(&request_body);

    match login_request {
        Ok(req) => {
            // Create auth service
            let auth_service = match AuthService::from_global_pool() {
                Ok(service) => service,
                Err(e) => {
                    ctx.set_response_status_code(500).await;
                    return LoginResponse::failure(format!("Service unavailable: {}", e.to_user_message()));
                }
            };

            // Perform login
            match auth_service.login(req).await {
                Ok(response) => {
                    if response.success {
                        ctx.set_response_status_code(200).await;
                    } else {
                        ctx.set_response_status_code(401).await;
                    }
                    response
                }
                Err(e) => {
                    if e.is_client_error() {
                        ctx.set_response_status_code(400).await;
                    } else {
                        ctx.set_response_status_code(500).await;
                    }
                    LoginResponse::failure(e.to_user_message())
                }
            }
        }
        Err(_) => {
            ctx.set_response_status_code(400).await;
            LoginResponse::failure("Invalid request format".to_string())
        }
    }
}

/// Handle logout request
async fn handle_logout(ctx: &Context) -> serde_json::Value {
    // Create auth service
    let auth_service = match AuthService::from_global_pool() {
        Ok(service) => service,
        Err(e) => {
            ctx.set_response_status_code(500).await;
            return serde_json::json!({
                "success": false,
                "message": format!("Service unavailable: {}", e.to_user_message())
            });
        }
    };

    // Extract session ID from request
    let session_id = match extract_session_id_from_context(ctx).await {
        Some(id) => id,
        None => {
            ctx.set_response_status_code(400).await;
            return serde_json::json!({
                "success": false,
                "message": "No session ID provided"
            });
        }
    };

    // Perform logout
    match auth_service.logout(&session_id) {
        Ok(_) => {
            ctx.set_response_status_code(200).await;
            serde_json::json!({
                "success": true,
                "message": "Logged out successfully"
            })
        }
        Err(e) => {
            ctx.set_response_status_code(400).await;
            serde_json::json!({
                "success": false,
                "message": e.to_user_message()
            })
        }
    }
}

/// Handle session info request
async fn handle_session_info(ctx: &Context) -> serde_json::Value {
    // Create auth service
    let auth_service = match AuthService::from_global_pool() {
        Ok(service) => service,
        Err(e) => {
            ctx.set_response_status_code(500).await;
            return serde_json::json!({
                "success": false,
                "message": format!("Service unavailable: {}", e.to_user_message())
            });
        }
    };

    // Extract session ID from request
    let session_id = match extract_session_id_from_context(ctx).await {
        Some(id) => id,
        None => {
            ctx.set_response_status_code(401).await;
            return serde_json::json!({
                "success": false,
                "message": "No session ID provided"
            });
        }
    };

    // Validate session
    match auth_service.validate_session(&session_id) {
        Ok(session_info) => {
            ctx.set_response_status_code(200).await;
            serde_json::json!({
                "success": true,
                "session": {
                    "session_id": session_info.session_id,
                    "user_id": session_info.user_id,
                    "username": session_info.username,
                    "expires_at": session_info.expires_at,
                    "last_accessed": session_info.last_accessed
                }
            })
        }
        Err(e) => {
            if e.is_client_error() {
                ctx.set_response_status_code(401).await;
            } else {
                ctx.set_response_status_code(500).await;
            }
            serde_json::json!({
                "success": false,
                "message": e.to_user_message()
            })
        }
    }
}

/// Extract session ID from context (simplified implementation)
async fn extract_session_id_from_context(ctx: &Context) -> Option<String> {
    // Try to get from request body for POST requests
    let request_body = ctx.get_request_body().await;
    if !request_body.is_empty() {
        if let Ok(body_json) = serde_json::from_slice::<serde_json::Value>(&request_body) {
            if let Some(session_id) = body_json.get("session_id") {
                if let Some(session_str) = session_id.as_str() {
                    return Some(session_str.to_string());
                }
            }
        }
    }

    // TODO: Implement proper header parsing when we understand the Context API better
    // For now, return None if not found in body
    None
}
