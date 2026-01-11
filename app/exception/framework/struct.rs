use super::*;

#[task_panic]
#[derive(Debug)]
pub struct TaskPanicHook {
    pub(super) content_type: String,
    pub(super) response_body: String,
}

#[request_error]
#[derive(Debug)]
pub struct RequestErrorHook {
    pub(super) response_status_code: ResponseStatusCode,
    pub(super) content_type: String,
    pub(super) response_body: String,
}
