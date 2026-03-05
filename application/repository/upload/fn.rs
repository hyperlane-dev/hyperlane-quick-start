use super::*;

#[instrument_trace]
fn get_file_id_map() -> &'static ArcRwLock<HashMapXxHash3_64<String, FileChunkData>> {
    FILE_ID_MAP.get_or_init(|| arc_rwlock(hash_map_xx_hash3_64()))
}

#[instrument_trace]
pub async fn read_file_id_map() -> RwLockReadGuard<'static, HashMapXxHash3_64<String, FileChunkData>>
{
    get_file_id_map().read().await
}

#[instrument_trace]
pub async fn write_file_id_map()
-> RwLockWriteGuard<'static, HashMapXxHash3_64<String, FileChunkData>> {
    get_file_id_map().write().await
}
