use super::*;

#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct FileChunkData {
    file_id: String,
    file_name: String,
    chunk_index: usize,
    total_chunks: usize,
    base_file_dir: String,
}

#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct RangeRequest {
    start: u64,
    end: Option<u64>,
}

#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct PartialContent {
    data: Vec<u8>,
    content_range: String,
    content_length: u64,
    total_size: u64,
}
