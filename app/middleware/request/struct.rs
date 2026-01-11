use super::*;

#[request_middleware(1)]
#[derive(Clone, Data, Debug, Default)]
pub struct HttpRequestMiddleware;

#[request_middleware(2)]
#[derive(Clone, Data, Debug, Default)]
pub struct CrossMiddleware;

#[request_middleware(3)]
#[derive(Clone, Data, Debug, Default)]
pub struct ResponseHeaderMiddleware;

#[request_middleware(4)]
#[derive(Clone, Data, Debug, Default)]
pub struct ResponseStatusCodeMiddleware;

#[request_middleware(5)]
#[derive(Clone, Data, Debug, Default)]
pub struct ResponseBodyMiddleware;

#[request_middleware(6)]
#[derive(Clone, Data, Debug, Default)]
pub struct OptionMethodMiddleware;

#[request_middleware(7)]
#[derive(Clone, Data, Debug, Default)]
pub struct UpgradeMiddleware;
