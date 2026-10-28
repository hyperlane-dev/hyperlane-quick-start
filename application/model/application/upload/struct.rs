use super::*;

/// Metadata for a single file chunk during a chunked file upload process.
#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct FileChunkData {
    /// The unique identifier for the file being uploaded.
    #[get_mut(pub(crate))]
    pub file_id: String,
    /// The original file name provided by the uploader.
    #[get_mut(pub(crate))]
    pub file_name: String,
    /// The zero-based index of this chunk in the upload sequence.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub chunk_index: usize,
    /// The total number of chunks that compose the complete file.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub total_chunks: usize,
    /// The base directory path where chunk data is stored on disk.
    #[get_mut(pub(crate))]
    pub base_file_dir: String,
}

/// Represents an HTTP Range request for partial content delivery.
#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct RangeRequest {
    /// The starting byte offset (inclusive) for the range request.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start: u64,
    #[get(pub(crate), type(copy))]
    /// The optional ending byte offset (inclusive) for the range request; if None, reads to end of file.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) end: Option<u64>,
}

/// Represents a partial content response for an HTTP Range request.
#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct PartialContent {
    /// The binary data content for the requested range.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) data: Vec<u8>,
    /// The Content-Range header value (e.g., "bytes 0-1023/2048").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content_range: String,
    /// The length of the returned content in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content_length: u64,
    /// The total size of the complete file in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total_size: u64,
}
