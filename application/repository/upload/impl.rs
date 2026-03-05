use super::*;

impl FileChunkRepository {
    #[instrument_trace]
    pub async fn add_file_id_map(data: &FileChunkData) {
        write_file_id_map()
            .await
            .insert(data.get_file_id().to_owned(), data.clone());
    }

    #[instrument_trace]
    pub async fn remove_file_id_map(file_id: &str) {
        write_file_id_map().await.remove(file_id);
    }

    #[instrument_trace]
    pub async fn get_merge_file_chunk_data(file_id: &str) -> Option<FileChunkData> {
        read_file_id_map().await.get(file_id).cloned()
    }
}
