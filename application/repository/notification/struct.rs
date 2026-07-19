use super::*;

/// Repository for performing database operations on the `notification` table.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationRepository;

/// Query parameters for filtering and paginating notification records.
#[derive(Clone, Data, Debug, Default)]
pub struct NotificationQuery {
    /// The optional user ID to filter notifications by owner.
    #[get(type(copy))]
    pub(super) user_id: Option<i32>,
    /// The optional notification type to filter by category.
    pub(super) notification_type: Option<String>,
    /// The optional read status filter (true = read, false = unread).
    #[get(type(copy))]
    pub(super) is_read: Option<bool>,
    /// The page number for pagination (1-based).
    #[get(type(copy))]
    pub(super) page: i32,
    /// The maximum number of records per page.
    #[get(type(copy))]
    pub(super) limit: u64,
}
