use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct MonitorService;

#[cfg(not(target_os = "windows"))]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct LinuxMemoryInfo {
    #[get(type(copy), pub(crate))]
    pub(super) total: u64,
    #[get(type(copy), pub(crate))]
    pub(super) available: u64,
    #[get(type(copy), pub(crate))]
    pub(super) free: u64,
    #[get(type(copy), pub(crate))]
    pub(super) buffers: u64,
    #[get(type(copy), pub(crate))]
    pub(super) cached: u64,
}
