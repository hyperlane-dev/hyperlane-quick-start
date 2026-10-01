/// Upload view redirect path.
pub const UPLOAD_VIEW_REDIRECT_PATH: &str = "/static/upload/index.html";
/// OpenAPI route for the upload page.
pub const ROUTE_UPLOAD: &str = "/upload";

/// Reason phrase for a successful response.
pub const HTTP_REASON_SUCCESS: &str = "Success";

/// Reason phrase for a malformed request.
pub const HTTP_REASON_BAD_REQUEST: &str = "Bad Request";

/// Reason phrase for a missing resource.
pub const HTTP_REASON_NOT_FOUND: &str = "Not Found";

/// Reason phrase for an unexpected server failure.
pub const HTTP_REASON_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// OpenAPI route for downloading one uploaded file.
pub const ROUTE_UPLOAD_FILE: &str = "/upload/file/{upload_dir}/{upload_file}";

/// Description of the upload directory route parameter.
pub const ROUTE_PARAM_DESC_UPLOAD_DIR: &str = "Upload directory";

/// Description of the uploaded file route parameter.
pub const ROUTE_PARAM_DESC_UPLOAD_FILE: &str = "Upload file name";
