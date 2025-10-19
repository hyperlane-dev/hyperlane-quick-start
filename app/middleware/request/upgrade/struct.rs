use super::*;

#[request_middleware(5)]
pub struct UpgradeMiddleware;
