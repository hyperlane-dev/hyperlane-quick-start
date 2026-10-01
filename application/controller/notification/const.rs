/// Maximum list limit.
pub const MAX_LIST_LIMIT: u64 = 100;

/// Error message when invalid notification id.
pub const ERROR_INVALID_NOTIFICATION_ID: &str = "Invalid notification ID";

/// Error message when notification id required.
pub const ERROR_NOTIFICATION_ID_REQUIRED: &str = "Notification ID is required";

/// Success notification marked as read.
pub const SUCCESS_NOTIFICATION_MARKED_AS_READ: &str = "Notification marked as read";

/// Success all notifications marked as read.
pub const SUCCESS_ALL_NOTIFICATIONS_MARKED_AS_READ: &str = "All notifications marked as read";

/// Success notification deleted.
pub const SUCCESS_NOTIFICATION_DELETED: &str = "Notification deleted successfully";

/// OpenAPI response description for bad request.
pub const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad request";

/// OpenAPI response description for internal server error.
pub const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal server error";

/// OpenAPI response description for notification all marked read.
pub const OPENAPI_DESCRIPTION_NOTIFICATION_ALL_MARKED_READ: &str =
    "All notifications marked as read successfully";

/// OpenAPI response description for notification created.
pub const OPENAPI_DESCRIPTION_NOTIFICATION_CREATED: &str = "Notification created successfully";

/// OpenAPI response description for notification details retrieved.
pub const OPENAPI_DESCRIPTION_NOTIFICATION_DETAILS_RETRIEVED: &str =
    "Notification details retrieved successfully";

/// OpenAPI response description for notification list retrieved.
pub const OPENAPI_DESCRIPTION_NOTIFICATION_LIST_RETRIEVED: &str =
    "List of notifications retrieved successfully";

/// OpenAPI response description for notification marked as read.
pub const OPENAPI_DESCRIPTION_NOTIFICATION_MARKED_AS_READ: &str =
    "Notification marked as read successfully";

/// OpenAPI response description for notification not found.
pub const OPENAPI_DESCRIPTION_NOTIFICATION_NOT_FOUND: &str = "Notification not found";

/// OpenAPI response description for notification unread count retrieved.
pub const OPENAPI_DESCRIPTION_NOTIFICATION_UNREAD_COUNT_RETRIEVED: &str =
    "Unread count retrieved successfully";

/// OpenAPI response description for unauthorized.
pub const OPENAPI_DESCRIPTION_UNAUTHORIZED: &str = "Unauthorized";

/// OpenAPI description of the notification id parameter.
pub const OPENAPI_PARAM_DESCRIPTION_NOTIFICATION_ID: &str = "Notification ID";

/// OpenAPI route path for the notification create endpoint.
pub const OPENAPI_PATH_NOTIFICATION_CREATE: &str = "/api/notification/create";

/// OpenAPI route path for the notification delete endpoint.
pub const OPENAPI_PATH_NOTIFICATION_DELETE: &str = "/api/notification/delete/{id}";

/// OpenAPI route path for the notification get endpoint.
pub const OPENAPI_PATH_NOTIFICATION_GET: &str = "/api/notification/get/{id}";

/// OpenAPI route path for the notification list endpoint.
pub const OPENAPI_PATH_NOTIFICATION_LIST: &str = "/api/notification/list";

/// OpenAPI route path for the notification mark read endpoint.
pub const OPENAPI_PATH_NOTIFICATION_MARK_READ: &str = "/api/notification/read/{id}";

/// OpenAPI route path for the notification read all endpoint.
pub const OPENAPI_PATH_NOTIFICATION_READ_ALL: &str = "/api/notification/read-all";

/// OpenAPI route path for the notification unread count endpoint.
pub const OPENAPI_PATH_NOTIFICATION_UNREAD_COUNT: &str = "/api/notification/unread-count";
