use super::*;

/// OpenAPI documentation endpoint for the root index route.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 302, description = OPENAPI_DESCRIPTION_INDEX_REDIRECT),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG_INDEX
)]
/// Body of the `openapi_index` free function.
#[instrument_trace]
pub fn openapi_index() {}
