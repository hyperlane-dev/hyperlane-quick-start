use super::*;

impl AuthController {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    /// Handle user registration
    pub async fn register(&self, ctx: &Context) -> serde_json::Value {
        // Parse request body
        let request_body = ctx.get_request_body().await;
        let register_request: Result<CreateUserRequest, _> = serde_json::from_slice(&request_body);

        match register_request {
            Ok(req) => {
                match self.auth_service.register(req).await {
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
    pub async fn change_password(&self, ctx: &Context, user_id: i64) -> serde_json::Value {
        // Parse request body
        let request_body = ctx.get_request_body().await;
        let change_request: Result<ChangePasswordRequest, _> = serde_json::from_slice(&request_body);

        match change_request {
            Ok(req) => {
                match self.auth_service.change_password(user_id, req).await {
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

    /// Handle user profile retrieval
    pub async fn get_profile(&self, ctx: &Context, user_id: i64) -> serde_json::Value {
        match self.auth_service.get_user_profile(user_id).await {
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

    /// Handle user profile update
    pub async fn update_profile(&self, ctx: &Context, user_id: i64) -> serde_json::Value {
        // Parse request body
        let request_body = ctx.get_request_body().await;
        let update_request: Result<UpdateUserRequest, _> = serde_json::from_slice(&request_body);

        match update_request {
            Ok(req) => {
                match self.auth_service.update_user_profile(user_id, req).await {
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

    /// Check username availability
    pub async fn check_username(&self, ctx: &Context, username: &str) -> serde_json::Value {
        match self.auth_service.is_username_available(username).await {
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

    /// Check email availability
    pub async fn check_email(&self, ctx: &Context, email: &str) -> serde_json::Value {
        match self.auth_service.is_email_available(email).await {
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
}
