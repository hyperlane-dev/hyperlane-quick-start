use super::*;

/// Hook for handling task panic events, capturing error details for the response.
#[task_panic]
#[derive(Clone, Data, Debug, Default)]
pub struct TaskPanicHook {
    /// The content type of the panic response body.
    pub(super) content_type: String,
    /// The response body containing the panic error message.
    pub(super) response_body: String,
}

/// Hook for handling request error events, capturing error details including the HTTP status code.
#[request_error]
#[derive(Clone, Data, Debug, Default)]
pub struct RequestErrorHook {
    /// The HTTP response status code associated with the error.
    #[get(type(copy))]
    pub(super) response_status_code: ResponseStatusCode,
    /// The content type of the error response body.
    pub(super) content_type: String,
    /// The response body containing the error message.
    pub(super) response_body: String,
}
