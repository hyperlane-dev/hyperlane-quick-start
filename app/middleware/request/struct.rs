use super::*;

#[request_middleware(1)]
pub struct HttpRequestMiddleware;

#[request_middleware(2)]
pub struct CrossMiddleware;

#[request_middleware(3)]
pub struct ResponseHeaderMiddleware;

#[request_middleware(4)]
pub struct ResponseStatusCodeMiddleware;

#[request_middleware(5)]
pub struct ResponseBodyMiddleware;

#[request_middleware(6)]
pub struct OptionMethodMiddleware;

#[request_middleware(7)]
pub struct UpgradeMiddleware;
