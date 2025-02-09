use crate::sync::Lazy;

static LOG_ERROR_QUEUE: Lazy<LogListArcLock> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));
