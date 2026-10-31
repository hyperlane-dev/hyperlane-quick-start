use super::*;

/// file id map.
pub(crate) static FILE_ID_MAP: OnceLock<ArcRwLock<HashMapXxHash3_64<String, FileChunkData>>> =
    OnceLock::new();
