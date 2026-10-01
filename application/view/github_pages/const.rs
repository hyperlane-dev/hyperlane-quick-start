/// OpenAPI route for a GitHub Pages site.
pub const ROUTE_GITHUB_PAGES: &str = "/github/pages/{owner}/{repository}";

/// Description of the repository owner route parameter.
pub const ROUTE_PARAM_DESC_OWNER: &str = "GitHub owner or organization name";

/// Description of the repository route parameter.
pub const ROUTE_PARAM_DESC_REPOSITORY: &str = "GitHub repository name";

/// Reason phrase for a malformed request.
pub const HTTP_REASON_BAD_REQUEST: &str = "Bad Request";

/// Reason phrase for an unexpected server failure.
pub const HTTP_REASON_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// Reason phrase for a missing resource.
pub const HTTP_REASON_NOT_FOUND: &str = "Not Found";

/// Reason phrase for a successful response.
pub const HTTP_REASON_SUCCESS: &str = "Success";

/// OpenAPI route for one asset of a GitHub Pages site.
pub const ROUTE_GITHUB_PAGES_ASSET: &str = "/github/pages/{owner}/{repository}/{path:.*}";

/// Description of the static resource path route parameter.
pub const ROUTE_PARAM_DESC_PATH: &str = "Resource path";
