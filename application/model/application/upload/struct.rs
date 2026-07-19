use super::*;

/// Metadata for a single file chunk during a chunked file upload process.
#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct FileChunkData {
    /// The unique identifier for the file being uploaded.
    pub(super) file_id: String,
    /// The original file name provided by the uploader.
    pub(super) file_name: String,
    /// The zero-based index of this chunk in the upload sequence.
    #[get(type(copy))]
    pub(super) chunk_index: usize,
    /// The total number of chunks that compose the complete file.
    #[get(type(copy))]
    pub(super) total_chunks: usize,
    /// The base directory path where chunk data is stored on disk.
    pub(super) base_file_dir: String,
}

/// Represents an HTTP Range request for partial content delivery.
#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct RangeRequest {
    /// The starting byte offset (inclusive) for the range request.
    #[get(type(copy))]
    pub(super) start: u64,
    /// The optional ending byte offset (inclusive) for the range request; if None, reads to end of file.
    pub(super) end: Option<u64>,
}

/// Represents a partial content response for an HTTP Range request.
#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct PartialContent {
    /// The binary data content for the requested range.
    pub(super) data: Vec<u8>,
    /// The Content-Range header value (e.g., "bytes 0-1023/2048").
    pub(super) content_range: String,
    /// The length of the returned content in bytes.
    #[get(type(copy))]
    pub(super) content_length: u64,
    /// The total size of the complete file in bytes.
    #[get(type(copy))]
    pub(super) total_size: u64,
}
