use super::*;

/// openapi online docs.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_ONLINE_DOCS,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
/// Body of the `openapi_online_docs` free function.
#[instrument_trace]
pub fn openapi_online_docs() {}
