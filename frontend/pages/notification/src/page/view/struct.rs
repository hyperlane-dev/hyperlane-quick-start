use super::*;

/// The props of the notification page.
#[derive(Clone, Copy, Debug, Default)]
pub struct NotificationPageProps {
    /// The state the page renders from.
    pub state: NotificationState,
}

/// The props of the list of notifications.
#[derive(Clone, Copy, Debug, Default)]
pub struct NotificationListProps {
    /// The state the page renders from.
    pub state: NotificationState,
}

/// The props of one row of the list of notifications.
#[derive(Clone, Debug, Default)]
pub struct NotificationRowProps {
    /// The state the page renders from.
    pub state: NotificationState,
    /// The notification the row shows.
    pub notification: NotificationItem,
}

/// The props of one metric tile.
#[derive(Clone, Debug, Default)]
pub struct StatTileProps {
    /// The name of the metric.
    pub label: &'static str,
    /// The value of the metric.
    pub value: String,
    /// The glyph of the metric.
    pub icon: &'static str,
}

/// The props of one button of a filter.
#[derive(Clone, Copy, Debug, Default)]
pub struct FilterButtonProps {
    /// The state the page renders from.
    pub state: NotificationState,
    /// The kind of the filter.
    pub kind: &'static str,
    /// The value of the filter.
    pub value: &'static str,
    /// The label of the button.
    pub label: &'static str,
}

/// The props of the detail of a notification.
#[derive(Clone, Copy, Debug, Default)]
pub struct NotificationDetailProps {
    /// The state the page renders from.
    pub state: NotificationState,
}

/// The props of the form that writes a notification.
#[derive(Clone, Copy, Debug, Default)]
pub struct NotificationFormProps {
    /// The state the page renders from.
    pub state: NotificationState,
}
