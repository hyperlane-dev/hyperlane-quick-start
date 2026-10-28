/// The unit string for HTTP Content-Range headers, always "bytes".
pub const BYTES_UNIT: &str = "bytes";

/// Error message when invalid directory or file name.
pub(crate) const ERROR_INVALID_DIRECTORY_OR_FILE_NAME: &str = "Invalid directory or file name";

/// Error message when file not found or empty.
pub(crate) const ERROR_FILE_NOT_FOUND_OR_EMPTY: &str = "File not found or empty";

/// Error message when invalid range header format.
pub(crate) const ERROR_INVALID_RANGE_HEADER_FORMAT: &str = "Invalid range header format";

/// Error message when invalid range specification.
pub(crate) const ERROR_INVALID_RANGE_SPECIFICATION: &str = "Invalid range specification";

/// Error message when invalid range both empty.
pub(crate) const ERROR_INVALID_RANGE_BOTH_EMPTY: &str =
    "Invalid range: both start and end are empty";

/// Error message when invalid end range.
pub(crate) const ERROR_INVALID_END_RANGE: &str = "Invalid end range";

/// Error message when invalid start range.
pub(crate) const ERROR_INVALID_START_RANGE: &str = "Invalid start range";

/// Error message when range start exceeds file size.
pub(crate) const ERROR_RANGE_START_EXCEEDS_FILE_SIZE: &str = "Range start exceeds file size";

/// Error message when invalid range start greater than end.
pub(crate) const ERROR_INVALID_RANGE_START_GREATER_THAN_END: &str = "Invalid range: start > end";

/// Error message when file not found.
pub(crate) const ERROR_FILE_NOT_FOUND: &str = "File not found";

/// Error message when file is empty.
pub(crate) const ERROR_FILE_IS_EMPTY: &str = "File is empty";

/// Prefix string for range header prefix.
pub(crate) const RANGE_HEADER_PREFIX: &str = "bytes=";

/// Error message when the request body carries no file identifier.
pub(crate) const ERROR_MISSING_FILE_ID: &str = "Missing file id";

/// Error message when the request body carries no number of chunks.
pub(crate) const ERROR_MISSING_TOTAL_CHUNKS: &str = "Missing total chunks";

/// Error message when the number of chunks of the request body is not a valid count.
pub(crate) const ERROR_INVALID_TOTAL_CHUNKS: &str = "Invalid total chunks";

/// Error message when the request body carries no file name.
pub(crate) const ERROR_MISSING_FILE_NAME: &str = "Missing file name";

/// Error message when the request body carries no chunk index.
pub(crate) const ERROR_MISSING_CHUNK_INDEX: &str = "Missing chunk index";

/// Error message when the chunk index of the request body is not a valid index.
pub(crate) const ERROR_INVALID_CHUNK_INDEX: &str = "Invalid chunk index";

/// Error message when the request body carries no uploaded file.
pub(crate) const ERROR_MISSING_FILE_PART: &str = "Missing file";

/// Media type accepted by the multipart upload endpoints.
pub(crate) const UPLOAD_FORM_CONTENT_TYPE: &str = "multipart/form-data";

/// Error message when the upload body cannot be parsed.
pub(crate) const ERROR_INVALID_MULTIPART: &str = "Invalid multipart form data";

/// Error message when the upload identifier is not a safe path component.
pub(crate) const ERROR_INVALID_FILE_ID: &str = "Invalid file id";

/// Error message when an upload session has not been registered.
pub(crate) const ERROR_UPLOAD_NOT_REGISTERED: &str = "Upload session not registered";
