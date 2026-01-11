use super::*;

#[response_middleware(1)]
#[derive(Debug)]
pub struct SendMiddleware;

#[response_middleware(2)]
#[derive(Debug)]
pub struct LogMiddleware;
