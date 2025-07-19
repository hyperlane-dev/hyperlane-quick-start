use super::*;

pub static FILE_ID_MAP: Lazy<ArcRwLock<HashMapXxHash3_64<String, FileChunkData>>> =
    Lazy::new(|| arc_rwlock(hash_map_xx_hash3_64()));
