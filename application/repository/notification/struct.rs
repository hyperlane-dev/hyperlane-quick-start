use super::*;

/// Repository for performing database operations on the `notification` table.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationRepository;

/// Query parameters for filtering and paginating notification records.
#[derive(Clone, Data, Debug, Default)]
pub struct NotificationQuery {
    /// The optional user ID to filter notifications by owner.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: Option<i32>,
    /// The optional notification type to filter by category.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) notification_type: Option<String>,
    /// The optional read status filter (true = read, false = unread).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_read: Option<bool>,
    /// The page number for pagination (1-based).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: i32,
    /// The maximum number of records per page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) limit: u64,
}
