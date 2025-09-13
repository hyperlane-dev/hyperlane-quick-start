use super::*;

#[request_middleware(1)]
#[response_version(HttpVersion::HTTP1_1)]
#[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
#[response_header(ACCESS_CONTROL_ALLOW_METHODS => ALL_METHODS)]
#[response_header(ACCESS_CONTROL_ALLOW_HEADERS => WILDCARD_ANY)]
pub async fn cross(ctx: Context) {}
