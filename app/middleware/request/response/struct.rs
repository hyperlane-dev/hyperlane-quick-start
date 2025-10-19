use super::*;

#[request_middleware(2)]
pub struct ResponseHeaderMiddleware;

#[request_middleware(3)]
pub struct ResponseStatusCodeMiddleware;

#[request_middleware(4)]
pub struct ResponseBodyMiddleware;
