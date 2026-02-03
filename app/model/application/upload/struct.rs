use super::*;

#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct FileChunkData {
    #[get(pub)]
    pub(super) file_id: String,
    #[get(pub)]
    pub(super) file_name: String,
    #[get(type(copy), pub)]
    pub(super) chunk_index: usize,
    #[get(type(copy), pub)]
    pub(super) total_chunks: usize,
    #[get(pub)]
    pub(super) base_file_dir: String,
}

#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct RangeRequest {
    #[get(type(copy), pub)]
    pub(super) start: u64,
    #[get(pub)]
    pub(super) end: Option<u64>,
}

#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct PartialContent {
    #[get(pub)]
    pub(super) data: Vec<u8>,
    #[get(pub)]
    pub(super) content_range: String,
    #[get(type(copy), pub)]
    pub(super) content_length: u64,
    #[get(type(copy), pub)]
    pub(super) total_size: u64,
}
