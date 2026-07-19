use super::*;

/// Implementation of methods for `FileChunkRepository`.
impl FileChunkRepository {
    /// Adds a file chunk data entry to the in-memory file ID map.
    ///
    /// # Arguments
    ///
    /// - `&FileChunkData`: The file chunk data containing the file ID and metadata.
    #[instrument_trace]
    pub async fn add_file_id_map(data: &FileChunkData) {
        write_file_id_map()
            .await
            .insert(data.get_file_id().to_owned(), data.clone());
    }

    /// Removes a file chunk data entry from the in-memory file ID map.
    ///
    /// # Arguments
    ///
    /// - `&str`: The file ID to remove.
    #[instrument_trace]
    pub async fn remove_file_id_map(file_id: &str) {
        write_file_id_map().await.remove(file_id);
    }

    /// Retrieves file chunk data by file ID from the in-memory map.
    ///
    /// # Arguments
    ///
    /// - `&str`: The file ID to look up.
    ///
    /// # Returns
    ///
    /// - `Option<FileChunkData>`: The file chunk data if found, or `None`.
    #[instrument_trace]
    pub async fn get_merge_file_chunk_data(file_id: &str) -> Option<FileChunkData> {
        read_file_id_map().await.get(file_id).cloned()
    }
}
