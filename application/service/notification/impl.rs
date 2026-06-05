use super::*;

impl NotificationService {
    #[instrument_trace]
    pub async fn create_notification(
        user_id: i32,
        request: CreateNotificationRequest,
    ) -> Result<NotificationResponse, String> {
        let active_model: NotificationActiveModel = NotificationActiveModel {
            user_id: ActiveValue::Set(user_id),
            title: ActiveValue::Set(request.get_title().clone()),
            content: ActiveValue::Set(request.get_content().clone()),
            notification_type: ActiveValue::Set(request.get_notification_type().clone()),
            is_read: ActiveValue::Set(false),
            is_deleted: ActiveValue::Set(false),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let model: NotificationModel = NotificationRepository::insert(active_model).await?;
        let response: NotificationResponse = Self::model_to_response(&model)?;
        Ok(response)
    }

    #[instrument_trace]
    pub async fn list_notifications(
        user_id: i32,
        query: NotificationListQueryRequest,
    ) -> Result<NotificationListResponse, String> {
        let page: i32 = query
            .get_page()
            .unwrap_or(DEFAULT_PAGE_NUMBER)
            .max(DEFAULT_PAGE_NUMBER);
        let limit: u64 = query
            .get_limit()
            .unwrap_or(DEFAULT_PAGE_LIMIT)
            .min(MAX_PAGE_LIMIT);
        let mut repository_query: NotificationQuery = NotificationQuery::default();
        repository_query
            .set_user_id(Some(user_id))
            .set_notification_type(query.try_get_notification_type().clone())
            .set_is_read(query.try_get_is_read())
            .set_page(page)
            .set_limit(limit);
        let (models, total): (Vec<NotificationModel>, i64) =
            NotificationRepository::query_with_pagination(repository_query).await?;
        let notifications: Vec<NotificationResponse> = models
            .iter()
            .map(|model: &NotificationModel| Self::model_to_response(model))
            .collect::<Result<Vec<NotificationResponse>, String>>()?;
        let mut response: NotificationListResponse = NotificationListResponse::default();
        response
            .set_notifications(notifications)
            .set_total(total)
            .set_page(page)
            .set_limit(limit);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn get_notification(
        notification_id: i32,
        user_id: i32,
    ) -> Result<Option<NotificationResponse>, String> {
        let model: Option<NotificationModel> =
            NotificationRepository::find_by_id(notification_id).await?;
        match model {
            Some(notification) => {
                if notification.get_user_id() != user_id {
                    return Err(ERROR_ACCESS_OWN_NOTIFICATIONS_ONLY.to_string());
                }
                let response: NotificationResponse = Self::model_to_response(&notification)?;
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    #[instrument_trace]
    pub async fn mark_as_read(notification_id: i32, user_id: i32) -> Result<(), String> {
        let model: Option<NotificationModel> =
            NotificationRepository::find_by_id(notification_id).await?;
        match model {
            Some(notification) => {
                if notification.get_user_id() != user_id {
                    return Err(ERROR_UPDATE_OWN_NOTIFICATIONS_ONLY.to_string());
                }
                NotificationRepository::update_read_status(notification_id, true).await
            }
            None => Err(ERROR_NOTIFICATION_NOT_FOUND.to_string()),
        }
    }

    #[instrument_trace]
    pub async fn mark_all_as_read(user_id: i32) -> Result<(), String> {
        let mut query: NotificationQuery = NotificationQuery::default();
        query
            .set_user_id(Some(user_id))
            .set_is_read(Some(false))
            .set_page(DEFAULT_PAGE_NUMBER)
            .set_limit(MARK_ALL_READ_LIMIT);
        let (models, _): (Vec<NotificationModel>, i64) =
            NotificationRepository::query_with_pagination(query).await?;
        for model in models {
            NotificationRepository::update_read_status(model.get_id(), true).await?;
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn delete_notification(notification_id: i32, user_id: i32) -> Result<(), String> {
        let model: Option<NotificationModel> =
            NotificationRepository::find_by_id(notification_id).await?;
        match model {
            Some(notification) => {
                if notification.get_user_id() != user_id {
                    return Err(ERROR_DELETE_OWN_NOTIFICATIONS_ONLY.to_string());
                }
                NotificationRepository::soft_delete_by_id(notification_id).await
            }
            None => Err(ERROR_NOTIFICATION_NOT_FOUND.to_string()),
        }
    }

    #[instrument_trace]
    pub async fn get_unread_count(user_id: i32) -> Result<i64, String> {
        NotificationRepository::count_unread(user_id).await
    }

    #[instrument_trace]
    fn model_to_response(model: &NotificationModel) -> Result<NotificationResponse, String> {
        let mut response: NotificationResponse = NotificationResponse::default();
        response
            .set_id(AuthService::encode_id(model.get_id()).unwrap_or_default())
            .set_user_id(AuthService::encode_id(model.get_user_id()).unwrap_or_default())
            .set_title(model.get_title().clone())
            .set_content(model.get_content().clone())
            .set_notification_type(model.get_notification_type().clone())
            .set_is_read(model.get_is_read())
            .set_created_at(
                model
                    .try_get_created_at()
                    .map(|dt: NaiveDateTime| dt.and_utc().timestamp_millis())
                    .unwrap_or(0),
            );
        Ok(response)
    }
}
