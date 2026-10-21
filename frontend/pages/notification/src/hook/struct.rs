use super::*;

/// The reactive state the notification page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct NotificationState {
    /// The notifications of the list being displayed.
    #[get(type(copy))]
    pub notifications: Signal<Vec<NotificationItem>>,
    /// The number of notifications the filter holds in total.
    #[get(type(copy))]
    pub total: Signal<i64>,
    /// The page being displayed.
    #[get(type(copy))]
    pub page: Signal<i32>,
    /// The number of notifications that have not been read.
    #[get(type(copy))]
    pub unread: Signal<i64>,
    /// The kind the list is filtered by.
    #[get(type(copy))]
    pub filter_type: Signal<String>,
    /// The read state the list is filtered by.
    #[get(type(copy))]
    pub filter_read: Signal<String>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The message of the last success.
    #[get(type(copy))]
    pub notice: Signal<String>,
    /// The notification whose detail is open.
    #[get(type(copy))]
    pub detail: Signal<Option<NotificationItem>>,
    /// Whether the form that writes a notification is open.
    #[get(type(copy))]
    pub create_open: Signal<bool>,
    /// The title of the notification that is written.
    #[get(type(copy))]
    pub create_title: Signal<String>,
    /// The body of the notification that is written.
    #[get(type(copy))]
    pub create_content: Signal<String>,
    /// The kind of the notification that is written.
    #[get(type(copy))]
    pub create_type: Signal<String>,
}
