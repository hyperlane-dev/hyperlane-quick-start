use super::*;

pub async fn read_file_id_map<'a>() -> RwLockReadGuard<'a, HashMapXxHash3_64<String, FileChunkData>>
{
    FILE_ID_MAP.read().await
}

pub async fn write_file_id_map<'a>()
-> RwLockWriteGuard<'a, HashMapXxHash3_64<String, FileChunkData>> {
    FILE_ID_MAP.write().await
}
