/// HTTP header key for save upload dir.
pub const HEADER_SAVE_UPLOAD_DIR: &str = "save_upload_dir";

/// The name of the form field carrying the identifier of the upload session.
pub(crate) const FIELD_FILE_ID: &str = "file_id";

/// The name of the form field carrying the number of chunks of the upload session.
pub(crate) const FIELD_TOTAL_CHUNKS: &str = "total_chunks";

/// The name of the form field carrying the index of the stored chunk.
pub(crate) const FIELD_CHUNK_INDEX: &str = "chunk_index";

/// The name of the form field carrying the directory of the upload.
pub(crate) const FIELD_DIRECTORY: &str = "directory_path";

/// The name of the form part carrying the uploaded file.
pub(crate) const PART_FILE: &str = "file";
