use super::*;

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

/// Handle user registration
async fn handle_register(ctx: &Context) -> serde_json::Value {
    // Parse request body
    let request_body = ctx.get_request_body().await;
    let register_request: Result<CreateUserRequest, _> = serde_json::from_slice(&request_body);

    match register_request {
        Ok(req) => {
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

            match auth_service.register(req).await {
                Ok(user_response) => {
                    ctx.set_response_status_code(201).await;
                    serde_json::json!({
                        "success": true,
                        "message": "User registered successfully",
                        "user": user_response
                    })
                }
                Err(e) => {
                    if e.is_client_error() {
                        ctx.set_response_status_code(400).await;
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
        Err(_) => {
            ctx.set_response_status_code(400).await;
            serde_json::json!({
                "success": false,
                "message": "Invalid request format"
            })
        }
    }
}

/// Handle password change
async fn handle_change_password(ctx: &Context) -> serde_json::Value {
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

    // Extract session ID and validate session
    let session_id = match extract_session_id_from_context(ctx).await {
        Some(id) => id,
        None => {
            ctx.set_response_status_code(401).await;
            return serde_json::json!({
                "success": false,
                "message": "Authentication required"
            });
        }
    };

    let session_info = match auth_service.validate_session(&session_id) {
        Ok(info) => info,
        Err(e) => {
            ctx.set_response_status_code(401).await;
            return serde_json::json!({
                "success": false,
                "message": e.to_user_message()
            });
        }
    };

    // Parse request body
    let request_body = ctx.get_request_body().await;
    let change_request: Result<ChangePasswordRequest, _> = serde_json::from_slice(&request_body);

    match change_request {
        Ok(req) => {
            match auth_service
                .change_password(session_info.user_id, req)
                .await
            {
                Ok(_) => {
                    ctx.set_response_status_code(200).await;
                    serde_json::json!({
                        "success": true,
                        "message": "Password changed successfully"
                    })
                }
                Err(e) => {
                    if e.is_client_error() {
                        ctx.set_response_status_code(400).await;
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
        Err(_) => {
            ctx.set_response_status_code(400).await;
            serde_json::json!({
                "success": false,
                "message": "Invalid request format"
            })
        }
    }
}

/// Handle get profile
async fn handle_get_profile(ctx: &Context) -> serde_json::Value {
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

    // Extract session ID and validate session
    let session_id = match extract_session_id_from_context(ctx).await {
        Some(id) => id,
        None => {
            ctx.set_response_status_code(401).await;
            return serde_json::json!({
                "success": false,
                "message": "Authentication required"
            });
        }
    };

    let session_info = match auth_service.validate_session(&session_id) {
        Ok(info) => info,
        Err(e) => {
            ctx.set_response_status_code(401).await;
            return serde_json::json!({
                "success": false,
                "message": e.to_user_message()
            });
        }
    };

    match auth_service.get_user_profile(session_info.user_id).await {
        Ok(user_response) => {
            ctx.set_response_status_code(200).await;
            serde_json::json!({
                "success": true,
                "user": user_response
            })
        }
        Err(e) => {
            if e.is_client_error() {
                ctx.set_response_status_code(404).await;
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

/// Handle update profile
async fn handle_update_profile(ctx: &Context) -> serde_json::Value {
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

    // Extract session ID and validate session
    let session_id = match extract_session_id_from_context(ctx).await {
        Some(id) => id,
        None => {
            ctx.set_response_status_code(401).await;
            return serde_json::json!({
                "success": false,
                "message": "Authentication required"
            });
        }
    };

    let session_info = match auth_service.validate_session(&session_id) {
        Ok(info) => info,
        Err(e) => {
            ctx.set_response_status_code(401).await;
            return serde_json::json!({
                "success": false,
                "message": e.to_user_message()
            });
        }
    };

    // Parse request body
    let request_body = ctx.get_request_body().await;
    let update_request: Result<UpdateUserRequest, _> = serde_json::from_slice(&request_body);

    match update_request {
        Ok(req) => {
            match auth_service
                .update_user_profile(session_info.user_id, req)
                .await
            {
                Ok(user_response) => {
                    ctx.set_response_status_code(200).await;
                    serde_json::json!({
                        "success": true,
                        "message": "Profile updated successfully",
                        "user": user_response
                    })
                }
                Err(e) => {
                    if e.is_client_error() {
                        ctx.set_response_status_code(400).await;
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
        Err(_) => {
            ctx.set_response_status_code(400).await;
            serde_json::json!({
                "success": false,
                "message": "Invalid request format"
            })
        }
    }
}

/// Handle check username
async fn handle_check_username(ctx: &Context) -> serde_json::Value {
    // Parse request body
    let request_body = ctx.get_request_body().await;
    let check_request: Result<serde_json::Value, _> = serde_json::from_slice(&request_body);

    let username = match check_request {
        Ok(json) => match json.get("username").and_then(|v| v.as_str()) {
            Some(username) => username.to_string(),
            None => {
                ctx.set_response_status_code(400).await;
                return serde_json::json!({
                    "success": false,
                    "message": "Username is required"
                });
            }
        },
        Err(_) => {
            ctx.set_response_status_code(400).await;
            return serde_json::json!({
                "success": false,
                "message": "Invalid request format"
            });
        }
    };

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

    match auth_service.is_username_available(&username).await {
        Ok(available) => {
            ctx.set_response_status_code(200).await;
            serde_json::json!({
                "success": true,
                "available": available,
                "username": username
            })
        }
        Err(e) => {
            ctx.set_response_status_code(500).await;
            serde_json::json!({
                "success": false,
                "message": e.to_user_message()
            })
        }
    }
}

/// Handle check email
async fn handle_check_email(ctx: &Context) -> serde_json::Value {
    // Parse request body
    let request_body = ctx.get_request_body().await;
    let check_request: Result<serde_json::Value, _> = serde_json::from_slice(&request_body);

    let email = match check_request {
        Ok(json) => match json.get("email").and_then(|v| v.as_str()) {
            Some(email) => email.to_string(),
            None => {
                ctx.set_response_status_code(400).await;
                return serde_json::json!({
                    "success": false,
                    "message": "Email is required"
                });
            }
        },
        Err(_) => {
            ctx.set_response_status_code(400).await;
            return serde_json::json!({
                "success": false,
                "message": "Invalid request format"
            });
        }
    };

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

    match auth_service.is_email_available(&email).await {
        Ok(available) => {
            ctx.set_response_status_code(200).await;
            serde_json::json!({
                "success": true,
                "available": available,
                "email": email
            })
        }
        Err(e) => {
            ctx.set_response_status_code(500).await;
            serde_json::json!({
                "success": false,
                "message": e.to_user_message()
            })
        }
    }
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

/// Handle session validation
async fn handle_validate_session(ctx: &Context) -> serde_json::Value {
    // Extract session ID from request
    let session_id = match extract_session_id_from_context(ctx).await {
        Some(id) => id,
        None => {
            ctx.set_response_status_code(401).await;
            return serde_json::json!({
                "success": false,
                "message": "No session provided"
            });
        }
    };

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

    // Validate session
    match auth_service.validate_session(&session_id) {
        Ok(session_info) => {
            ctx.set_response_status_code(200).await;
            serde_json::json!({
                "success": true,
                "user_id": session_info.user_id,
                "username": session_info.username,
                "expires_at": session_info.expires_at
            })
        }
        Err(e) => {
            ctx.set_response_status_code(401).await;
            serde_json::json!({
                "success": false,
                "message": e.to_user_message()
            })
        }
    }
}

/// Extract session ID from context (shared function)
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
