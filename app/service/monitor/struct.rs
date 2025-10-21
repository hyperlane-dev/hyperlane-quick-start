pub struct MonitorService;

#[cfg(not(target_os = "windows"))]
struct LinuxMemoryInfo {
    total: u64,
    available: u64,
    free: u64,
    buffers: u64,
    cached: u64,
}
