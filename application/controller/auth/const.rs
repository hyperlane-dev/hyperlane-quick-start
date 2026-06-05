pub const COOKIE_MAX_AGE_SECONDS: u64 = 86400;

pub const COOKIE_CLEAR_FORMAT: &str = "token=; Path=/; Max-Age=0; HttpOnly";

pub const ERROR_INVALID_USER_ID: &str = "Invalid user ID";

pub const ERROR_USER_ID_REQUIRED: &str = "User ID is required";

pub const ERROR_UPDATE_OWN_DATA_ONLY: &str = "You can only update your own data";

pub const ERROR_ONLY_ADMIN_CAN_DELETE: &str = "Only admin can delete users";

pub const ERROR_CANNOT_DELETE_YOURSELF: &str = "Cannot delete yourself";

pub const SUCCESS_USER_DELETED: &str = "User deleted successfully";

pub const SUCCESS_LOGGED_OUT: &str = "Logged out successfully";
