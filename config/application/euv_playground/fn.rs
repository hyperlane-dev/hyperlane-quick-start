use super::*;

/// Returns the `TOPIC_EUV_PLAYGROUND_BUILD` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn topic_euv_playground_build() -> &'static str {
    TOPIC_EUV_PLAYGROUND_BUILD
}

/// Returns the `CONSUMER_GROUP_BUILD_WORKER` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn consumer_group_build_worker() -> &'static str {
    CONSUMER_GROUP_BUILD_WORKER
}

/// Returns the `BUILD_TASK_SEPARATOR` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn build_task_separator() -> &'static str {
    BUILD_TASK_SEPARATOR
}

/// Returns the `BUILD_JOB_TTL_MS` configuration value.
///
/// # Returns
///
/// - `i64` - The configured value.
pub fn build_job_ttl_ms() -> i64 {
    BUILD_JOB_TTL_MS
}

/// Returns the `BUILD_JOB_GC_INTERVAL_MS` configuration value.
///
/// # Returns
///
/// - `u64` - The configured value.
pub fn build_job_gc_interval_ms() -> u64 {
    BUILD_JOB_GC_INTERVAL_MS
}

/// Returns the `BUILD_TOPIC_CAPACITY` configuration value.
///
/// # Returns
///
/// - `usize` - The configured value.
pub fn build_topic_capacity() -> usize {
    BUILD_TOPIC_CAPACITY
}
