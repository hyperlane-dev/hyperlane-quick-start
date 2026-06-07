use super::*;

/// Database access methods for `NotificationRepository`.
impl NotificationRepository {
    /// Inserts a new notification record into the database.
    ///
    /// # Arguments
    ///
    /// - `NotificationActiveModel`: The active model containing the notification data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<NotificationModel, String>`: The inserted notification model.
    #[instrument_trace]
    pub async fn insert(
        active_model: NotificationActiveModel,
    ) -> Result<NotificationModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: NotificationModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds a non-deleted notification by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The notification identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<NotificationModel>, String>`: The notification model if found and not deleted, or `None`.
    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<NotificationModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<NotificationModel> = NotificationEntity::find_by_id(id)
            .filter(NotificationColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Queries non-deleted notifications with pagination, filtering by user, type, and read status.
    ///
    /// # Arguments
    ///
    /// - `NotificationQuery`: The query parameters including user, type, read status, and pagination.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<NotificationModel>, i64), String>`: The paginated notifications and total count.
    #[instrument_trace]
    pub async fn query_with_pagination(
        query: NotificationQuery,
    ) -> Result<(Vec<NotificationModel>, i64), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: sea_orm::Select<NotificationEntity> = NotificationEntity::find();
        if let Some(user_id) = query.try_get_user_id() {
            base_select = base_select.filter(NotificationColumn::UserId.eq(user_id));
        }
        if let Some(notification_type) = query.try_get_notification_type() {
            base_select =
                base_select.filter(NotificationColumn::NotificationType.eq(notification_type));
        }
        if let Some(is_read) = query.try_get_is_read() {
            base_select = base_select.filter(NotificationColumn::IsRead.eq(is_read));
        }
        base_select = base_select.filter(NotificationColumn::IsDeleted.eq(false));
        let total: i64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i64;
        let records: Vec<NotificationModel> = base_select
            .order_by_desc(NotificationColumn::CreatedAt)
            .offset(((query.get_page() - 1) as i64 * query.get_limit() as i64) as u64)
            .limit(query.get_limit())
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok((records, total))
    }

    /// Updates the read status of a notification by its identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The notification identifier.
    /// - `bool`: The new read status.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the notification is not found.
    #[instrument_trace]
    pub async fn update_read_status(id: i32, is_read: bool) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let notification: NotificationModel = NotificationEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Notification not found".to_string())?;
        let mut active_model: NotificationActiveModel = notification.into();
        active_model.is_read = ActiveValue::Set(is_read);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Soft-deletes a notification by setting its `is_deleted` flag to true.
    ///
    /// # Arguments
    ///
    /// - `i32`: The notification identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the notification is not found.
    #[instrument_trace]
    pub async fn soft_delete_by_id(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let notification: NotificationModel = NotificationEntity::find_by_id(id)
            .filter(NotificationColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Notification not found".to_string())?;
        let mut active_model: NotificationActiveModel = notification.into();
        active_model.is_deleted = ActiveValue::Set(true);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Counts the number of unread and non-deleted notifications for a user.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<i64, String>`: The count of unread notifications.
    #[instrument_trace]
    pub async fn count_unread(user_id: i32) -> Result<i64, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let count: i64 = NotificationEntity::find()
            .filter(NotificationColumn::UserId.eq(user_id))
            .filter(NotificationColumn::IsRead.eq(false))
            .filter(NotificationColumn::IsDeleted.eq(false))
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i64;
        Ok(count)
    }
}
