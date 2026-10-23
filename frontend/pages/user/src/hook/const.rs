/// The endpoint of the users of the server.
pub(crate) const PATH_LIST: &str = "/api/user/list";

/// The endpoint of one user.
pub(crate) const PATH_GET: &str = "/api/user/get";

/// The endpoint that changes the contact fields of a user.
pub(crate) const PATH_UPDATE: &str = "/api/user/update";

/// The endpoint that changes the password of a user.
pub(crate) const PATH_CHANGE_PASSWORD: &str = "/api/user/change_password";

/// The endpoint that changes the status of a user.
pub(crate) const PATH_UPDATE_STATUS: &str = "/api/user/update_status";

/// The endpoint that removes a user.
pub(crate) const PATH_DELETE: &str = "/api/user/delete";

/// The empty value of a field.
pub(crate) const EMPTY: &str = "";

/// The number of users one page holds.
pub(crate) const PAGE_SIZE: u64 = 20;

/// The first page of the list.
pub(crate) const PAGE_FIRST: i32 = 1;

/// The number of milliseconds the search waits before it is sent.
pub(crate) const SEARCH_DELAY_MS: i32 = 400;

/// The view of the detail that shows the profile of a user.
pub(crate) const MODE_VIEW: &str = "view";

/// The view of the detail that changes the contact fields of a user.
pub(crate) const MODE_EDIT: &str = "edit";

/// The view of the detail that changes the password of a user.
pub(crate) const MODE_PASSWORD: &str = "password";

/// The role of an administrator.
pub(crate) const ROLE_ADMIN: &str = "admin";

/// The status of a user that waits for the approval.
pub(crate) const STATUS_PENDING: &str = "pending";

/// The status of a user that is approved.
pub(crate) const STATUS_APPROVED: &str = "approved";

/// The status of a user that is rejected.
pub(crate) const STATUS_REJECTED: &str = "rejected";

/// The failure reported when a password field is empty.
pub(crate) const ERROR_EMPTY_PASSWORD: &str = "Please fill in both passwords";
