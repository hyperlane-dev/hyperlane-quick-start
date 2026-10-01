use super::*;

/// openapi euv playground view.
#[utoipa::path(
    get,
    path = ROUTE_EUV_PLAYGROUND,
    responses(
        (status = 302, description = RESPONSE_DESCRIPTION_REDIRECT),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    ),
    tag = OPENAPI_TAG
)]
#[instrument_trace]
pub fn openapi_euv_playground_view() {}
