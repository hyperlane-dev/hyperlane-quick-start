/// Error message when invalid directory or file name.
pub const ERROR_INVALID_DIRECTORY_OR_FILE_NAME: &str = "Invalid directory or file name";

/// Error message when file not found or empty.
pub const ERROR_FILE_NOT_FOUND_OR_EMPTY: &str = "File not found or empty";

/// Error message when invalid range header format.
pub const ERROR_INVALID_RANGE_HEADER_FORMAT: &str = "Invalid range header format";

/// Error message when invalid range specification.
pub const ERROR_INVALID_RANGE_SPECIFICATION: &str = "Invalid range specification";

/// Error message when invalid range both empty.
pub const ERROR_INVALID_RANGE_BOTH_EMPTY: &str = "Invalid range: both start and end are empty";

/// Error message when invalid end range.
pub const ERROR_INVALID_END_RANGE: &str = "Invalid end range";

/// Error message when invalid start range.
pub const ERROR_INVALID_START_RANGE: &str = "Invalid start range";

/// Error message when range start exceeds file size.
pub const ERROR_RANGE_START_EXCEEDS_FILE_SIZE: &str = "Range start exceeds file size";

/// Error message when invalid range start greater than end.
pub const ERROR_INVALID_RANGE_START_GREATER_THAN_END: &str = "Invalid range: start > end";

/// Error message when file not found.
pub const ERROR_FILE_NOT_FOUND: &str = "File not found";

/// Error message when file is empty.
pub const ERROR_FILE_IS_EMPTY: &str = "File is empty";

/// Prefix string for range header prefix.
pub const RANGE_HEADER_PREFIX: &str = "bytes=";

/// The unit string for HTTP Content-Range headers, always "bytes".
pub const BYTES_UNIT: &str = "bytes";
