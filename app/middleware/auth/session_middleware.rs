use super::*;
use crate::service::auth::*;
use hyperlane::*;
use std::sync::Arc;

pub struct SessionMiddleware {
    auth_service: Arc<AuthService>,
}

impl SessionMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    /// Extract session ID from request headers or cookies
    /// For now, this is a simplified implementation
    pub async fn extract_session_id(_ctx: &Context) -> Option<String> {
        // TODO: Implement proper header parsing when we understand the Context API better
        // For now, return None to allow compilation
        None
    }

    /// Validate session and set user context
    pub async fn validate_session(&self, ctx: &Context) -> Result<SessionInfo, AuthError> {
        let session_id = Self::extract_session_id(ctx).await
            .ok_or_else(|| AuthError::SessionError(SessionError::InvalidSessionId))?;

        self.auth_service.validate_session(&session_id)
    }

    /// Middleware function to require authentication
    pub async fn require_auth(&self, ctx: &Context) -> Result<SessionInfo, AuthError> {
        self.validate_session(ctx).await
    }

    /// Middleware function for optional authentication
    pub async fn optional_auth(&self, ctx: &Context) -> Option<SessionInfo> {
        self.validate_session(ctx).await.ok()
    }
}

/// Helper function to create unauthorized response
pub async fn unauthorized_response(ctx: &Context, message: &str) {
    ctx.set_response_status_code(401).await;
    ctx.set_response_header("Content-Type", "application/json").await;
    
    let error_response = serde_json::json!({
        "error": "Unauthorized",
        "message": message,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    let response_body = serde_json::to_vec(&error_response).unwrap_or_default();
    ctx.set_response_body(&response_body).await;
}

/// Helper function to create forbidden response
pub async fn forbidden_response(ctx: &Context, message: &str) {
    ctx.set_response_status_code(403).await;
    ctx.set_response_header("Content-Type", "application/json").await;
    
    let error_response = serde_json::json!({
        "error": "Forbidden",
        "message": message,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    let response_body = serde_json::to_vec(&error_response).unwrap_or_default();
    ctx.set_response_body(&response_body).await;
}