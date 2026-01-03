use super::*;

#[task_panic]
pub struct TaskPanicHook {
    pub(super) content_type: String,
    pub(super) response_body: String,
}

#[request_error]
pub struct RequestErrorHook {
    pub(super) response_status_code: ResponseStatusCode,
    pub(super) content_type: String,
    pub(super) response_body: String,
}
