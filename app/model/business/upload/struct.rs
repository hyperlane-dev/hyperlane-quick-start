use super::*;

#[derive(Debug, Default, Data, Clone, ToSchema)]
pub struct FileChunkData {
    file_id: String,
    file_name: String,
    chunk_index: usize,
    total_chunks: usize,
    base_file_dir: String,
}

#[derive(Debug, Clone, ToSchema)]
pub struct RangeRequest {
    pub start: u64,
    pub end: Option<u64>,
}

#[derive(Debug, Clone, ToSchema)]
pub struct PartialContent {
    pub data: Vec<u8>,
    pub content_range: String,
    pub content_length: u64,
    pub total_size: u64,
}
