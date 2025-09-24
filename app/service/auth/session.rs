use super::*;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: i64,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub user_id: i64,
    pub username: String,
    pub expires_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
}

pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    session_timeout: Duration,
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("Session inactive")]
    SessionInactive,
    
    #[error("Invalid session ID")]
    InvalidSessionId,
    
    #[error("Session storage error")]
    StorageError,
}

impl SessionManager {
    pub fn new(session_timeout_hours: i64) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_timeout: Duration::hours(session_timeout_hours),
        }
    }

    /// Create a new session for a user
    pub fn create_session(&self, user_id: i64, username: String) -> Result<SessionInfo, SessionError> {
        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + self.session_timeout;

        let session = Session {
            session_id: session_id.clone(),
            user_id,
            username: username.clone(),
            created_at: now,
            expires_at,
            last_accessed: now,
            is_active: true,
        };

        let mut sessions = self.sessions.write().map_err(|_| SessionError::StorageError)?;
        sessions.insert(session_id.clone(), session);

        Ok(SessionInfo {
            session_id,
            user_id,
            username,
            expires_at,
            last_accessed: now,
        })
    }

    /// Validate and refresh a session
    pub fn validate_session(&self, session_id: &str) -> Result<SessionInfo, SessionError> {
        if session_id.is_empty() {
            return Err(SessionError::InvalidSessionId);
        }

        let mut sessions = self.sessions.write().map_err(|_| SessionError::StorageError)?;
        
        let session = sessions.get_mut(session_id).ok_or(SessionError::SessionNotFound)?;

        if !session.is_active {
            return Err(SessionError::SessionInactive);
        }

        let now = Utc::now();
        if now > session.expires_at {
            session.is_active = false;
            return Err(SessionError::SessionExpired);
        }

        // Refresh session
        session.last_accessed = now;
        session.expires_at = now + self.session_timeout;

        Ok(SessionInfo {
            session_id: session.session_id.clone(),
            user_id: session.user_id,
            username: session.username.clone(),
            expires_at: session.expires_at,
            last_accessed: session.last_accessed,
        })
    }

    /// Invalidate a session (logout)
    pub fn invalidate_session(&self, session_id: &str) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().map_err(|_| SessionError::StorageError)?;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.is_active = false;
        }

        Ok(())
    }

    /// Invalidate all sessions for a user
    pub fn invalidate_user_sessions(&self, user_id: i64) -> Result<usize, SessionError> {
        let mut sessions = self.sessions.write().map_err(|_| SessionError::StorageError)?;
        let mut invalidated_count = 0;

        for session in sessions.values_mut() {
            if session.user_id == user_id && session.is_active {
                session.is_active = false;
                invalidated_count += 1;
            }
        }

        Ok(invalidated_count)
    }

    /// Get all active sessions for a user
    pub fn get_user_sessions(&self, user_id: i64) -> Result<Vec<SessionInfo>, SessionError> {
        let sessions = self.sessions.read().map_err(|_| SessionError::StorageError)?;
        let now = Utc::now();

        let user_sessions: Vec<SessionInfo> = sessions
            .values()
            .filter(|session| {
                session.user_id == user_id 
                && session.is_active 
                && now <= session.expires_at
            })
            .map(|session| SessionInfo {
                session_id: session.session_id.clone(),
                user_id: session.user_id,
                username: session.username.clone(),
                expires_at: session.expires_at,
                last_accessed: session.last_accessed,
            })
            .collect();

        Ok(user_sessions)
    }

    /// Clean up expired sessions
    pub fn cleanup_expired_sessions(&self) -> Result<usize, SessionError> {
        let mut sessions = self.sessions.write().map_err(|_| SessionError::StorageError)?;
        let now = Utc::now();
        let mut removed_count = 0;

        sessions.retain(|_, session| {
            if now > session.expires_at {
                removed_count += 1;
                false
            } else {
                true
            }
        });

        Ok(removed_count)
    }

    /// Get session statistics
    pub fn get_session_stats(&self) -> Result<SessionStats, SessionError> {
        let sessions = self.sessions.read().map_err(|_| SessionError::StorageError)?;
        let now = Utc::now();

        let total_sessions = sessions.len();
        let active_sessions = sessions.values()
            .filter(|s| s.is_active && now <= s.expires_at)
            .count();
        let expired_sessions = sessions.values()
            .filter(|s| now > s.expires_at)
            .count();

        Ok(SessionStats {
            total_sessions,
            active_sessions,
            expired_sessions,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct SessionStats {
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub expired_sessions: usize,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(24) // 24 hours default session timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let manager = SessionManager::new(1);
        let session_info = manager.create_session(1, "testuser".to_string()).unwrap();
        
        assert!(!session_info.session_id.is_empty());
        assert_eq!(session_info.user_id, 1);
        assert_eq!(session_info.username, "testuser");
    }

    #[test]
    fn test_session_validation() {
        let manager = SessionManager::new(1);
        let session_info = manager.create_session(1, "testuser".to_string()).unwrap();
        
        let validated = manager.validate_session(&session_info.session_id).unwrap();
        assert_eq!(validated.user_id, 1);
        assert_eq!(validated.username, "testuser");
    }

    #[test]
    fn test_session_invalidation() {
        let manager = SessionManager::new(1);
        let session_info = manager.create_session(1, "testuser".to_string()).unwrap();
        
        manager.invalidate_session(&session_info.session_id).unwrap();
        
        let result = manager.validate_session(&session_info.session_id);
        assert!(matches!(result, Err(SessionError::SessionInactive)));
    }
}