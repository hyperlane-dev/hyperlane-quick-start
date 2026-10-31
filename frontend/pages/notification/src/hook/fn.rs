use super::*;

/// Creates the reactive state the notification page renders from.
///
/// The state holds the list of notifications, the two filters, the number of
/// the notifications that have not been read and the fields of the form that
/// writes a notification. The list is loaded as soon as the page is mounted.
///
/// # Returns
///
/// - `NotificationState` - The state the page renders from.
pub fn use_notification_state() -> NotificationState {
    let notifications: Signal<Vec<NotificationItem>> = App::use_signal(Vec::new);
    let total: Signal<i64> = App::use_signal(|| 0);
    let page: Signal<i32> = App::use_signal(|| PAGE_FIRST);
    let unread: Signal<i64> = App::use_signal(|| 0);
    let filter_type: Signal<String> = App::use_signal(|| FILTER_ALL.to_string());
    let filter_read: Signal<String> = App::use_signal(|| FILTER_ALL.to_string());
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let notice: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let detail: Signal<Option<NotificationItem>> = App::use_signal(|| None);
    let create_open: Signal<bool> = App::use_signal(|| false);
    let create_title: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let create_content: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let create_type: Signal<String> = App::use_signal(|| TYPE_SYSTEM.to_string());
    let state: NotificationState = NotificationState {
        notifications,
        total,
        page,
        unread,
        filter_type,
        filter_read,
        loading,
        error,
        notice,
        detail,
        create_open,
        create_title,
        create_content,
        create_type,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_unread_count(state));
        spawn_local(load_notifications(state));
        true
    });
    state
}

/// Loads the notifications of the page being displayed.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
pub async fn load_notifications(state: NotificationState) {
    let page: i32 = state.get_page().get();
    let kind: String = state.get_filter_type().get();
    let read: String = state.get_filter_read().get();
    state.get_loading().set(true);
    let mut path: String = format!("{PATH_LIST}?page={page}&limit={PAGE_SIZE}");
    if !kind.is_empty() {
        path.push_str(&format!("&notification_type={kind}"));
    }
    if !read.is_empty() {
        path.push_str(&format!("&is_read={read}"));
    }
    match get_json::<NotificationList>(&path).await {
        Ok(list) => {
            state.get_notifications().set(list.notifications);
            state.get_total().set(list.total);
            state.get_page().set(list.page);
            state.get_error().set(EMPTY.to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
    state.get_loading().set(false);
}

/// Loads the number of notifications that have not been read.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The number has been written into the state.
pub async fn load_unread_count(state: NotificationState) {
    match get_json::<UnreadCount>(PATH_UNREAD_COUNT).await {
        Ok(count) => state.get_unread().set(count.count),
        Err(error) => state.get_error().set(error.message),
    }
}

/// Marks one notification as read.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
/// - `String` - The identifier of the notification.
///
/// # Returns
///
/// - `()` - The notification has been read, or the failure has been reported.
pub async fn mark_as_read(state: NotificationState, id: String) {
    let path: String = format!("{PATH_READ}/{id}");
    match request_json::<serde_json::Value>(HttpMethod::Post, &path, None).await {
        Ok(_) => {
            state.get_notice().set("Marked as read".to_string());
            load_unread_count(state).await;
            load_notifications(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Marks every notification as read.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The notifications have been read, or the failure has been reported.
pub async fn mark_all_as_read(state: NotificationState) {
    match request_json::<serde_json::Value>(HttpMethod::Post, PATH_READ_ALL, None).await {
        Ok(_) => {
            state
                .get_notice()
                .set("All notifications marked as read".to_string());
            load_unread_count(state).await;
            load_notifications(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Removes one notification.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
/// - `String` - The identifier of the notification.
///
/// # Returns
///
/// - `()` - The notification has been removed, or the failure has been reported.
pub async fn delete_notification(state: NotificationState, id: String) {
    let path: String = format!("{PATH_DELETE}/{id}");
    match request_json::<serde_json::Value>(HttpMethod::Post, &path, None).await {
        Ok(_) => {
            state.get_notice().set("Notification deleted".to_string());
            load_unread_count(state).await;
            load_notifications(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Creates the notification the form holds.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The notification has been created, or the failure has been reported.
pub async fn create_notification(state: NotificationState) {
    let title: String = state.get_create_title().get().trim().to_string();
    let content: String = state.get_create_content().get().trim().to_string();
    let kind: String = state.get_create_type().get();
    if title.is_empty() {
        state.get_error().set(ERROR_EMPTY_TITLE.to_string());
        return;
    }
    if content.is_empty() {
        state.get_error().set(ERROR_EMPTY_CONTENT.to_string());
        return;
    }
    let body: CreateNotificationRequest = CreateNotificationRequest {
        title,
        content,
        notification_type: kind,
    };
    match post_json::<NotificationItem, CreateNotificationRequest>(PATH_CREATE, &body).await {
        Ok(_) => {
            state.get_create_title().set(EMPTY.to_string());
            state.get_create_content().set(EMPTY.to_string());
            state.get_create_type().set(TYPE_SYSTEM.to_string());
            state.get_create_open().set(false);
            state.get_notice().set("Notification created".to_string());
            state.get_page().set(PAGE_FIRST);
            load_unread_count(state).await;
            load_notifications(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Opens the detail of a notification.
///
/// A notification that has not been read yet is marked as read, the same way
/// the hand written page did it.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
/// - `NotificationItem` - The notification whose detail is opened.
///
/// # Returns
///
/// - `()` - The detail has been written into the state.
pub fn open_detail(state: NotificationState, notification: NotificationItem) {
    let id: String = notification.id.clone();
    let unread: bool = !notification.is_read;
    state.get_detail().set(Some(notification));
    if unread {
        spawn_local(mark_as_read(state, id));
    }
}
