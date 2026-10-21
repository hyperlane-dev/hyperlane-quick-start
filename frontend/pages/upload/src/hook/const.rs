/// The endpoint that registers an upload session.
pub(crate) const PATH_REGISTER: &str = "/api/upload/register";

/// The endpoint that stores one chunk of an upload session.
pub(crate) const PATH_SAVE: &str = "/api/upload/save";

/// The endpoint that merges the chunks of an upload session.
pub(crate) const PATH_MERGE: &str = "/api/upload/merge";

/// The header carrying the identifier of the upload session.
pub(crate) const HEADER_FILE_ID: &str = "x-file-id";

/// The header carrying the number of chunks of the upload session.
pub(crate) const HEADER_TOTAL_CHUNKS: &str = "x-total-chunks";

/// The header carrying the name of the uploaded file.
pub(crate) const HEADER_FILE_NAME: &str = "x-file-name";

/// The header carrying the index of the stored chunk.
pub(crate) const HEADER_CHUNK_INDEX: &str = "x-chunk-index";

/// The size of one chunk of an upload.
pub(crate) const CHUNK_SIZE: i32 = 512 * 1024;

/// The key the upload history is kept under in the browser storage.
pub(crate) const STORAGE_KEY: &str = "hyperlane_upload_history";

/// The name of the file the export of the history is written to.
pub(crate) const EXPORT_FILE_NAME: &str = "upload_history.json";

/// The empty text of the page.
pub(crate) const TEXT_EMPTY: &str = "";

/// The share of a file that marks the upload as finished.
pub(crate) const PERCENT_FULL: f64 = 100.0;

/// The number of bytes in a kilobyte.
pub(crate) const BYTES_PER_KILOBYTE: f64 = 1_024.0;

/// The number of bytes in a megabyte.
pub(crate) const BYTES_PER_MEGABYTE: f64 = 1_048_576.0;

/// The number of bytes in a gigabyte.
pub(crate) const BYTES_PER_GIGABYTE: f64 = 1_073_741_824.0;

/// The HTTP method name of a create request.
pub(crate) const METHOD_POST: &str = "POST";

/// The business status code that marks a successful call.
pub(crate) const STATUS_SUCCESS: i32 = 200;

/// The message reported when the browser window is not available.
pub(crate) const MESSAGE_NO_WINDOW: &str = "the browser window is not available";

/// The message reported when the request could not be sent.
pub(crate) const MESSAGE_REQUEST_FAILED: &str = "the request could not be sent";

/// The message reported when the response payload is not valid.
pub(crate) const MESSAGE_INVALID_PAYLOAD: &str = "the response payload is not valid";

/// The separator between the failure prefix and its cause.
pub(crate) const MESSAGE_SEPARATOR: &str = ": ";

/// The content type of a JSON file.
pub(crate) const CONTENT_TYPE_JSON: &str = "application/json";
