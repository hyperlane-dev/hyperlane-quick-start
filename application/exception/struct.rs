use super::*;

#[task_panic]
#[derive(Clone, Data, Debug, Default)]
pub struct TaskPanicHook {
    #[get(pub(crate))]
    pub(super) content_type: String,
    #[get(pub(crate))]
    pub(super) response_body: String,
}

#[request_error]
#[derive(Clone, Data, Debug, Default)]
pub struct RequestErrorHook {
    #[get(type(copy), pub(crate))]
    pub(super) response_status_code: ResponseStatusCode,
    #[get(pub(crate))]
    pub(super) content_type: String,
    #[get(pub(crate))]
    pub(super) response_body: String,
}
