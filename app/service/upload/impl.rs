use super::*;

impl FileChunkData {
    pub fn new(
        file_id: String,
        file_name: String,
        chunk_index: usize,
        total_chunks: usize,
        base_file_dir: String,
    ) -> Self {
        let mut data: FileChunkData = FileChunkData::default();
        data.set_file_id(file_id)
            .set_file_name(file_name)
            .set_chunk_index(chunk_index)
            .set_total_chunks(total_chunks)
            .set_base_file_dir(base_file_dir);
        data
    }
}

impl<'a> Default for UploadResponse<'a> {
    fn default() -> Self {
        Self {
            code: 100,
            url: "",
            msg: "",
        }
    }
}
