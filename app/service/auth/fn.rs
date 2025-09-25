use super::*;

/// Handle login request
pub async fn handle_login(ctx: &Context) -> LoginResponse {
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
                    return LoginResponse::failure(format!(
                        "Service unavailable: {}",
                        e.to_user_message()
                    ));
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
pub async fn handle_logout(ctx: &Context) -> serde_json::Value {
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
pub async fn handle_session_info(ctx: &Context) -> serde_json::Value {
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
pub async fn extract_session_id_from_context(ctx: &Context) -> Option<String> {
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

/// Handle user registration
pub async fn handle_register(ctx: &Context) -> serde_json::Value {
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
pub async fn handle_change_password(ctx: &Context) -> serde_json::Value {
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
pub async fn handle_get_profile(ctx: &Context) -> serde_json::Value {
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
pub async fn handle_update_profile(ctx: &Context) -> serde_json::Value {
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
pub async fn handle_check_username(ctx: &Context) -> serde_json::Value {
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
pub async fn handle_check_email(ctx: &Context) -> serde_json::Value {
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

/// Handle session validation
pub async fn handle_validate_session(ctx: &Context) -> serde_json::Value {
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
