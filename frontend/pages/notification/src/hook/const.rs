/// The endpoint of the notifications of the server.
pub(crate) const PATH_LIST: &str = "/api/notification/list";

/// The endpoint that marks one notification as read.
pub(crate) const PATH_READ: &str = "/api/notification/read";

/// The endpoint that marks every notification as read.
pub(crate) const PATH_READ_ALL: &str = "/api/notification/read-all";

/// The endpoint that removes a notification.
pub(crate) const PATH_DELETE: &str = "/api/notification/delete";

/// The endpoint that creates a notification.
pub(crate) const PATH_CREATE: &str = "/api/notification/create";

/// The endpoint of the number of notifications that have not been read.
pub(crate) const PATH_UNREAD_COUNT: &str = "/api/notification/unread-count";

/// The empty value of a field.
pub(crate) const EMPTY: &str = "";

/// The number of notifications one page holds.
pub(crate) const PAGE_SIZE: u64 = 20;

/// The first page of the list.
pub(crate) const PAGE_FIRST: i32 = 1;

/// The filter that holds every notification.
pub(crate) const FILTER_ALL: &str = "";

/// The filter that holds the notifications that have not been read.
pub(crate) const FILTER_UNREAD: &str = "false";

/// The filter that holds the notifications that have been read.
pub(crate) const FILTER_READ: &str = "true";

/// The kind of the filter that holds the notifications of one kind.
pub(crate) const FILTER_TYPE: &str = "type";

/// The kind of the filter that holds the notifications of one read state.
pub(crate) const FILTER_READ_KIND: &str = "read";

/// The kind of the filter that holds the kind of the notification that is written.
pub(crate) const FILTER_CREATE_TYPE: &str = "create-type";

/// The kind of a system notification.
pub(crate) const TYPE_SYSTEM: &str = "system";

/// The kind of a message notification.
pub(crate) const TYPE_MESSAGE: &str = "message";

/// The kind of an alert notification.
pub(crate) const TYPE_ALERT: &str = "alert";

/// The failure reported when the title of a notification is empty.
pub(crate) const ERROR_EMPTY_TITLE: &str = "Title is required";

/// The failure reported when the body of a notification is empty.
pub(crate) const ERROR_EMPTY_CONTENT: &str = "Content is required";
