use super::*;
use crate::model::persistent::user::*;
use chrono::{DateTime, Utc};
use hyperlane_config::framework::database::*;
use std::sync::Arc;
use tokio_postgres::Row;

impl PostgresUserRepository {
    pub fn new(pool: Arc<DatabaseConnectionPool>) -> Self {
        Self { pool }
    }

    pub fn from_global_pool() -> Result<Self, UserRepositoryError> {
        let pool = get_global_pool().map_err(UserRepositoryError::PoolError)?;
        Ok(Self::new(pool))
    }

    fn row_to_user(&self, row: &Row) -> Result<User, UserRepositoryError> {
        let created_at: DateTime<Utc> = row
            .try_get("created_at")
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;
        let updated_at: DateTime<Utc> = row
            .try_get("updated_at")
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(User::new(
            row.try_get("id")
                .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?,
            row.try_get("username")
                .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?,
            row.try_get("password_hash")
                .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?,
            row.try_get("email")
                .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?,
            created_at,
            updated_at,
            row.try_get("is_active")
                .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?,
        ))
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, UserRepositoryError> {
        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let rows = conn.query(
            "SELECT id, username, password_hash, email, created_at, updated_at, is_active FROM users WHERE id = $1",
            &[&id],
        ).await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.first() {
            Ok(Some(self.row_to_user(row)?))
        } else {
            Ok(None)
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserRepositoryError> {
        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let rows = conn.query(
            "SELECT id, username, password_hash, email, created_at, updated_at, is_active FROM users WHERE username = $1",
            &[&username],
        ).await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.first() {
            Ok(Some(self.row_to_user(row)?))
        } else {
            Ok(None)
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let rows = conn.query(
            "SELECT id, username, password_hash, email, created_at, updated_at, is_active FROM users WHERE email = $1",
            &[&email],
        ).await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.first() {
            Ok(Some(self.row_to_user(row)?))
        } else {
            Ok(None)
        }
    }

    async fn create_user(&self, request: CreateUserRequest) -> Result<User, UserRepositoryError> {
        request
            .validate()
            .map_err(UserRepositoryError::ValidationError)?;

        // Check if username already exists
        if self.find_by_username(&request.username).await?.is_some() {
            return Err(UserRepositoryError::UsernameAlreadyExists);
        }

        // Check if email already exists (if provided)
        if let Some(ref email) = request.email {
            if self.find_by_email(email).await?.is_some() {
                return Err(UserRepositoryError::EmailAlreadyExists);
            }
        }

        // Hash password
        let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|e| UserRepositoryError::PasswordHashingError(e.to_string()))?;

        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let rows = conn
            .query(
                r#"
            INSERT INTO users (username, password_hash, email, is_active)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, password_hash, email, created_at, updated_at, is_active
            "#,
                &[
                    &request.username,
                    &password_hash,
                    &request.email.as_ref(),
                    &true,
                ],
            )
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.first() {
            self.row_to_user(row)
        } else {
            Err(UserRepositoryError::DatabaseError(
                "Failed to create user".to_string(),
            ))
        }
    }

    async fn update_user(
        &self,
        id: i64,
        request: UpdateUserRequest,
    ) -> Result<User, UserRepositoryError> {
        request
            .validate()
            .map_err(UserRepositoryError::ValidationError)?;

        // Check if email already exists (if provided and different)
        if let Some(ref email) = request.email {
            if let Some(existing_user) = self.find_by_email(email).await? {
                if existing_user.id != id {
                    return Err(UserRepositoryError::EmailAlreadyExists);
                }
            }
        }

        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let mut query_parts = Vec::new();
        let mut param_count = 1;

        if let Some(ref _email) = request.email {
            if !query_parts.is_empty() {
                query_parts.push(", ".to_string());
            }
            query_parts.push(format!("email = ${}", param_count));
            param_count += 1;
        }

        if request.is_active.is_some() {
            if !query_parts.is_empty() {
                query_parts.push(", ".to_string());
            }
            query_parts.push(format!("is_active = ${}", param_count));
            param_count += 1;
        }

        if query_parts.is_empty() {
            return Err(UserRepositoryError::NoFieldsToUpdate);
        }

        query_parts.push(format!("updated_at = NOW()"));

        let final_query = format!(
            "UPDATE users SET {} WHERE id = ${} RETURNING id, username, password_hash, email, created_at, updated_at, is_active",
            query_parts.join(""),
            param_count
        );

        // Execute query with proper parameters
        let rows = if let Some(ref email) = request.email {
            if let Some(is_active) = request.is_active {
                conn.query(&final_query, &[email, &is_active, &id]).await
            } else {
                conn.query(&final_query, &[email, &id]).await
            }
        } else if let Some(is_active) = request.is_active {
            conn.query(&final_query, &[&is_active, &id]).await
        } else {
            conn.query(&final_query, &[&id]).await
        }
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.first() {
            self.row_to_user(row)
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }

    async fn delete_user(&self, id: i64) -> Result<bool, UserRepositoryError> {
        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let rows_affected = conn
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows_affected > 0)
    }

    async fn verify_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<bool, UserRepositoryError> {
        let user = self.find_by_username(username).await?;

        if let Some(user) = user {
            if !user.is_active {
                return Ok(false);
            }

            bcrypt::verify(password, &user.password_hash)
                .map_err(|e| UserRepositoryError::PasswordVerificationError(e.to_string()))
        } else {
            Ok(false)
        }
    }

    async fn change_password(
        &self,
        id: i64,
        new_password_hash: &str,
    ) -> Result<(), UserRepositoryError> {
        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let rows_affected = conn
            .execute(
                "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
                &[&new_password_hash, &id],
            )
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        if rows_affected > 0 {
            Ok(())
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }

    async fn list_users(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<User>, UserRepositoryError> {
        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let limit = limit.unwrap_or(50).min(100); // Max 100 users per request
        let offset = offset.unwrap_or(0);

        let rows = conn.query(
            "SELECT id, username, password_hash, email, created_at, updated_at, is_active FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        ).await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let mut users = Vec::new();
        for row in rows {
            users.push(self.row_to_user(&row)?);
        }

        Ok(users)
    }

    async fn count_users(&self) -> Result<i64, UserRepositoryError> {
        let conn = self
            .pool
            .get_connection()
            .await
            .map_err(|e| UserRepositoryError::ConnectionError(e.to_string()))?;

        let rows = conn
            .query("SELECT COUNT(*) FROM users", &[])
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.first() {
            let count: i64 = row
                .try_get(0)
                .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;
            Ok(count)
        } else {
            Ok(0)
        }
    }
}
