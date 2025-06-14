use super::*;

#[derive(Debug, Default, Data, Clone)]
pub struct FileChunkData {
    file_id: String,
    file_name: String,
    chunk_index: usize,
    total_chunks: usize,
    base_file_dir: String,
}

#[derive(Debug, Serialize, Data, Clone)]
pub struct UploadResponse<'a> {
    pub code: i32,
    pub url: &'a str,
    pub msg: &'a str,
}
