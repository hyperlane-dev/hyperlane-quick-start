/// Error message when invalid user id.
pub const ERROR_INVALID_USER_ID: &str = "Invalid user ID";

/// Error message when user id required.
pub const ERROR_USER_ID_REQUIRED: &str = "User ID is required";

/// Error message when update own data only.
pub const ERROR_UPDATE_OWN_DATA_ONLY: &str = "You can only update your own data";

/// Error message when only admin can delete.
pub const ERROR_ONLY_ADMIN_CAN_DELETE: &str = "Only admin can delete users";

/// Error message when cannot delete yourself.
pub const ERROR_CANNOT_DELETE_YOURSELF: &str = "Cannot delete yourself";

/// Success user deleted.
pub const SUCCESS_USER_DELETED: &str = "User deleted successfully";

/// OpenAPI response description for bad request.
pub const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad request";

/// OpenAPI response description for forbidden.
pub const OPENAPI_DESCRIPTION_FORBIDDEN: &str = "Forbidden";

/// OpenAPI response description for internal server error.
pub const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal server error";

/// OpenAPI response description for unauthorized.
pub const OPENAPI_DESCRIPTION_UNAUTHORIZED: &str = "Unauthorized";

/// OpenAPI response description for user details retrieved.
pub const OPENAPI_DESCRIPTION_USER_DETAILS_RETRIEVED: &str = "User details retrieved successfully";

/// OpenAPI response description for user list retrieved.
pub const OPENAPI_DESCRIPTION_USER_LIST_RETRIEVED: &str = "List of users retrieved successfully";

/// OpenAPI response description for user not found.
pub const OPENAPI_DESCRIPTION_USER_NOT_FOUND: &str = "User not found";

/// OpenAPI response description for user password changed.
pub const OPENAPI_DESCRIPTION_USER_PASSWORD_CHANGED: &str = "Password changed successfully";

/// OpenAPI response description for user status updated.
pub const OPENAPI_DESCRIPTION_USER_STATUS_UPDATED: &str = "User status updated successfully";

/// OpenAPI response description for user updated.
pub const OPENAPI_DESCRIPTION_USER_UPDATED: &str = "User updated successfully";

/// OpenAPI description of the user id parameter.
pub const OPENAPI_PARAM_DESCRIPTION_USER_ID: &str = "User ID";

/// OpenAPI route path for the user change password endpoint.
pub const OPENAPI_PATH_USER_CHANGE_PASSWORD: &str = "/api/user/change_password/{id}";

/// OpenAPI route path for the user delete endpoint.
pub const OPENAPI_PATH_USER_DELETE: &str = "/api/user/delete/{id}";

/// OpenAPI route path for the user get endpoint.
pub const OPENAPI_PATH_USER_GET: &str = "/api/user/get/{id}";

/// OpenAPI route path for the user list endpoint.
pub const OPENAPI_PATH_USER_LIST: &str = "/api/user/list";

/// OpenAPI route path for the user update endpoint.
pub const OPENAPI_PATH_USER_UPDATE: &str = "/api/user/update/{id}";

/// OpenAPI route path for the user update status endpoint.
pub const OPENAPI_PATH_USER_UPDATE_STATUS: &str = "/api/user/update_status/{id}";

/// Query parameter key for keyword.
pub const QUERY_KEY_KEYWORD: &str = "keyword";

/// Query parameter key for last id.
pub const QUERY_KEY_LAST_ID: &str = "last_id";

/// Query parameter key for limit.
pub const QUERY_KEY_LIMIT: &str = "limit";
