use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CicdService;

pub(super) struct StepOutputBuilder {
    pub(super) stdout: String,
    pub(super) stderr: String,
    pub(super) is_timeout: bool,
    pub(super) timeout_secs: u64,
}
