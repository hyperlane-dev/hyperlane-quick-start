/// Directory path for euv docs view redirect path.
///
/// Resolved by `application/view/github_pages` against the
/// `euv-dev/euv-docs` repository (see
/// `config/application/github_pages/const::SYNC_REPOSITORIES` for the
/// pre-fetch list). The trailing slash is intentional: GitHub Pages
/// serves the repository root via the slash form, and the proxy router
/// in `view/github_pages` redirects directory lookups there with a 301.
pub const EUV_DOCS_VIEW_REDIRECT_PATH: &str = "/github/pages/euv-dev/euv-docs/";
/// OpenAPI route path for the euv docs view.
pub const ROUTE_EUV_DOCS: &str = "/euv-docs";

/// Reason phrase for a successful response.
pub const HTTP_REASON_SUCCESS: &str = "Success";

/// Reason phrase for a malformed request.
pub const HTTP_REASON_BAD_REQUEST: &str = "Bad Request";

/// Reason phrase for a missing resource.
pub const HTTP_REASON_NOT_FOUND: &str = "Not Found";

/// Reason phrase for an unexpected server failure.
pub const HTTP_REASON_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
