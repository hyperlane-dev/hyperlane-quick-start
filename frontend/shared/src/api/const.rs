/// The HTTP method name of a read request.
pub(crate) const METHOD_GET: &str = "GET";

/// The HTTP method name of a create request.
pub(crate) const METHOD_POST: &str = "POST";

/// The HTTP method name of an update request.
pub(crate) const METHOD_PUT: &str = "PUT";

/// The HTTP method name of a delete request.
pub(crate) const METHOD_DELETE: &str = "DELETE";

/// The header the content type is sent in.
pub(crate) const HEADER_CONTENT_TYPE: &str = "Content-Type";

/// The content type every JSON request carries.
pub(crate) const CONTENT_TYPE_JSON: &str = "application/json";

/// The business status code that marks a successful call.
pub(crate) const STATUS_SUCCESS: i32 = 200;

/// The failure reported when the browser window is not available.
pub(crate) const MESSAGE_NO_WINDOW: &str = "the browser window is not available";

/// The failure reported when the request could not be sent.
pub(crate) const MESSAGE_REQUEST_FAILED: &str = "the request could not be sent";

/// The failure reported when the response body could not be read.
pub(crate) const MESSAGE_RESPONSE_FAILED: &str = "the response could not be read";

/// The failure reported when the response body is not a valid envelope.
pub(crate) const MESSAGE_INVALID_PAYLOAD: &str = "the response payload is not valid";

/// The separator between the failure prefix and its cause.
pub(crate) const MESSAGE_SEPARATOR: &str = ": ";
