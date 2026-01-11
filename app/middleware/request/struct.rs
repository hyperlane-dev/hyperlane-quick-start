use super::*;

#[request_middleware(1)]
#[derive(Debug)]
pub struct HttpRequestMiddleware;

#[request_middleware(2)]
#[derive(Debug)]
pub struct CrossMiddleware;

#[request_middleware(3)]
#[derive(Debug)]
pub struct ResponseHeaderMiddleware;

#[request_middleware(4)]
#[derive(Debug)]
pub struct ResponseStatusCodeMiddleware;

#[request_middleware(5)]
#[derive(Debug)]
pub struct ResponseBodyMiddleware;

#[request_middleware(6)]
#[derive(Debug)]
pub struct OptionMethodMiddleware;

#[request_middleware(7)]
#[derive(Debug)]
pub struct UpgradeMiddleware;
