use super::*;

#[derive(Debug, Default, Data, Clone, ToSchema)]
pub struct FileChunkData {
    file_id: String,
    file_name: String,
    chunk_index: usize,
    total_chunks: usize,
    base_file_dir: String,
}

#[derive(Debug, Clone, ToSchema, Data, Default)]
pub struct RangeRequest {
    start: u64,
    end: Option<u64>,
}

#[derive(Debug, Default, Clone, ToSchema, Data)]
pub struct PartialContent {
    data: Vec<u8>,
    content_range: String,
    content_length: u64,
    total_size: u64,
}
