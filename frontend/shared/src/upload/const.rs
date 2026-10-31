/// The failure reported when the multipart body could not be built.
pub(crate) const MESSAGE_FORM_FAILED: &str = "the multipart body could not be built";

/// The failure reported when the file could not be read.
pub(crate) const MESSAGE_FILE_UNREADABLE: &str = "the file could not be read";

/// The name of the form field carrying the identifier of the upload session.
pub(crate) const FIELD_FILE_ID: &str = "file_id";

/// The name of the form field carrying the number of chunks of the upload session.
pub(crate) const FIELD_TOTAL_CHUNKS: &str = "total_chunks";

/// The name of the form field carrying the index of the stored chunk.
pub(crate) const FIELD_CHUNK_INDEX: &str = "chunk_index";

/// The name of the form part carrying the uploaded file.
pub(crate) const PART_FILE: &str = "file";
