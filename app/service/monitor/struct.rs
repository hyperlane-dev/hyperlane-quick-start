#[cfg(not(target_os = "windows"))]
use super::*;

pub struct MonitorService;

#[cfg(not(target_os = "windows"))]
#[derive(Data, Default)]
pub struct LinuxMemoryInfo {
    total: u64,
    available: u64,
    free: u64,
    buffers: u64,
    cached: u64,
}
