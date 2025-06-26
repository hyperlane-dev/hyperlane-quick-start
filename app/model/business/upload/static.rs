use super::*;

pub static FILE_ID_MAP: Lazy<DashMapXxHash3_64<String, FileChunkData>> =
    Lazy::new(|| dash_map_xx_hash3_64());
