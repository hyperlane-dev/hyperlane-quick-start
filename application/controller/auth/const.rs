/// Maximum cookie age seconds.
pub const COOKIE_MAX_AGE_SECONDS: u64 = 86400;

/// Cookie format for format.
pub const COOKIE_CLEAR_FORMAT: &str = "token=; Path=/; Max-Age=0; HttpOnly";

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

/// Success logged out.
pub const SUCCESS_LOGGED_OUT: &str = "Logged out successfully";

/// OpenAPI response description for auth rsa public key retrieved.
pub const OPENAPI_DESCRIPTION_AUTH_RSA_PUBLIC_KEY_RETRIEVED: &str =
    "RSA public key retrieved successfully";

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

/// OpenAPI response description for user logged in.
pub const OPENAPI_DESCRIPTION_USER_LOGGED_IN: &str = "User logged in successfully";

/// OpenAPI response description for user not found.
pub const OPENAPI_DESCRIPTION_USER_NOT_FOUND: &str = "User not found";

/// OpenAPI response description for user password changed.
pub const OPENAPI_DESCRIPTION_USER_PASSWORD_CHANGED: &str = "Password changed successfully";

/// OpenAPI response description for user registered.
pub const OPENAPI_DESCRIPTION_USER_REGISTERED: &str = "User registered successfully";

/// OpenAPI response description for user status updated.
pub const OPENAPI_DESCRIPTION_USER_STATUS_UPDATED: &str = "User status updated successfully";

/// OpenAPI response description for user updated.
pub const OPENAPI_DESCRIPTION_USER_UPDATED: &str = "User updated successfully";

/// OpenAPI description of the user id parameter.
pub const OPENAPI_PARAM_DESCRIPTION_USER_ID: &str = "User ID";

/// OpenAPI route path for the auth login endpoint.
pub const OPENAPI_PATH_AUTH_LOGIN: &str = "/api/auth/login";

/// OpenAPI route path for the auth register endpoint.
pub const OPENAPI_PATH_AUTH_REGISTER: &str = "/api/auth/register";

/// OpenAPI route path for the auth rsa public key endpoint.
pub const OPENAPI_PATH_AUTH_RSA_PUBLIC_KEY: &str = "/api/auth/rsa/public-key";

/// OpenAPI route path for the auth user change password endpoint.
pub const OPENAPI_PATH_AUTH_USER_CHANGE_PASSWORD: &str = "/api/auth/user/change_password/{id}";

/// OpenAPI route path for the auth user delete endpoint.
pub const OPENAPI_PATH_AUTH_USER_DELETE: &str = "/api/auth/user/delete/{id}";

/// OpenAPI route path for the auth user get endpoint.
pub const OPENAPI_PATH_AUTH_USER_GET: &str = "/api/auth/user/get/{id}";

/// OpenAPI route path for the auth user list endpoint.
pub const OPENAPI_PATH_AUTH_USER_LIST: &str = "/api/auth/user/list";

/// OpenAPI route path for the auth user update endpoint.
pub const OPENAPI_PATH_AUTH_USER_UPDATE: &str = "/api/auth/user/update/{id}";

/// OpenAPI route path for the auth user update status endpoint.
pub const OPENAPI_PATH_AUTH_USER_UPDATE_STATUS: &str = "/api/auth/user/update_status/{id}";

/// Query parameter key for keyword.
pub const QUERY_KEY_KEYWORD: &str = "keyword";

/// Query parameter key for last id.
pub const QUERY_KEY_LAST_ID: &str = "last_id";

/// Query parameter key for limit.
pub const QUERY_KEY_LIMIT: &str = "limit";
