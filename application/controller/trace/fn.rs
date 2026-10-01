use super::*;

/// openapi trace search.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_TRACE,
    params(
        ("TRACE" = String, Path, description = OPENAPI_PARAM_DESCRIPTION_TRACE_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_trace_search() {}
