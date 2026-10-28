/// Default chat history limit.
pub(crate) const DEFAULT_CHAT_HISTORY_LIMIT: u64 = 20;

/// OpenAPI response description for bad request.
pub(crate) const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad Request";

/// OpenAPI response description for internal server error.
pub(crate) const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// OpenAPI response description for not found.
pub(crate) const OPENAPI_DESCRIPTION_NOT_FOUND: &str = "Not Found";

/// OpenAPI response description for success.
pub(crate) const OPENAPI_DESCRIPTION_SUCCESS: &str = "Success";

/// OpenAPI route path for the chat users online endpoint.
pub(crate) const OPENAPI_PATH_CHAT_USERS_ONLINE: &str = "/chat/users/online";

/// Query parameter key for before id.
pub(crate) const QUERY_KEY_BEFORE_ID: &str = "before_id";

/// Query parameter key for limit.
pub(crate) const QUERY_KEY_LIMIT: &str = "limit";
