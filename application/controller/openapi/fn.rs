use super::*;

/// openapi openapi json.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_OPENAPI_JSON,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_openapi_json() {}
