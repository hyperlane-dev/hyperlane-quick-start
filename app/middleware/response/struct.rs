use super::*;

#[response_middleware(1)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SendMiddleware;

#[response_middleware(2)]
#[derive(Clone, Copy, Debug, Default)]
pub struct LogMiddleware;
