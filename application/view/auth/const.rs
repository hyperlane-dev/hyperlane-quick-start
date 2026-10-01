/// Directory path for auth view redirect path.
pub const AUTH_VIEW_REDIRECT_PATH: &str = "/static/auth/index.html";

/// Prefix string for auth view redirect query prefix.
pub const AUTH_VIEW_REDIRECT_QUERY_PREFIX: &str = "?location=";
/// OpenAPI route path for the auth view.
pub const ROUTE_AUTH: &str = "/auth";

/// Response description for a redirect emitted by this route.
pub const RESPONSE_DESCRIPTION_REDIRECT: &str = "Redirect to auth page";

/// Response description for a server error emitted by this route.
pub const RESPONSE_DESCRIPTION_SERVER_ERROR: &str = "Internal Server Error";
