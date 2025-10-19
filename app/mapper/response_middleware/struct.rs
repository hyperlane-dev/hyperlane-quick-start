use super::*;

#[response_middleware(1)]
pub struct SendMiddleware;

#[response_middleware(2)]
pub struct LogMiddleware;
