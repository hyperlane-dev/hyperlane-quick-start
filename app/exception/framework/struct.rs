use super::*;

#[panic]
pub struct ServerPanic {
    pub(super) content_type: String,
    pub(super) response_body: String,
}

#[request_error]
pub struct ServerRequestError {
    pub(super) response_status_code: ResponseStatusCode,
    pub(super) content_type: String,
    pub(super) response_body: String,
}
