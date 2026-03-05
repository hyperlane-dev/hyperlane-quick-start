use super::*;

pub static FILE_ID_MAP: OnceLock<ArcRwLock<HashMapXxHash3_64<String, FileChunkData>>> =
    OnceLock::new();
