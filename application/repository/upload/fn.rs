use super::*;

/// Returns the global file id map, initializing it lazily.
///
/// # Returns
///
/// - `&'static ArcRwLock<HashMapXxHash3_64<String, FileChunkData>>` - The global file id map.
#[instrument_trace]
fn get_file_id_map() -> &'static ArcRwLock<HashMapXxHash3_64<String, FileChunkData>> {
    FILE_ID_MAP.get_or_init(|| arc_rwlock(hash_map_xx_hash3_64()))
}

/// Acquires a read guard over the global file id map.
///
/// # Returns
///
/// - `RwLockReadGuard<'static, HashMapXxHash3_64<String, FileChunkData>>` - The read guard.
#[instrument_trace]
pub async fn read_file_id_map() -> RwLockReadGuard<'static, HashMapXxHash3_64<String, FileChunkData>>
{
    get_file_id_map().read().await
}

/// write file id map.
#[instrument_trace]
pub async fn write_file_id_map()
-> RwLockWriteGuard<'static, HashMapXxHash3_64<String, FileChunkData>> {
    get_file_id_map().write().await
}
