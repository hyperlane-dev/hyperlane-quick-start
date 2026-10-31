use super::*;

/// Hook for handling task panic events, capturing error details for the response.
#[task_panic]
#[derive(Clone, Data, Debug, Default)]
pub struct TaskPanicHook {
    /// The content type of the panic response body.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content_type: String,
    /// The response body containing the panic error message.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) response_body: String,
}

/// Hook for handling request error events, capturing error details including the HTTP status code.
#[request_error]
#[derive(Clone, Data, Debug, Default)]
pub struct RequestErrorHook {
    /// The HTTP response status code associated with the error.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) response_status_code: ResponseStatusCode,
    /// The content type of the error response body.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content_type: String,
    /// The response body containing the error message.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) response_body: String,
}
