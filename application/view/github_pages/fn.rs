use super::*;

/// openapi github pages proxy root.
#[utoipa::path(
    get,
    path = ROUTE_GITHUB_PAGES,
    params(
        ("owner" = String, Path, description = ROUTE_PARAM_DESC_OWNER),
        ("repository" = String, Path, description = ROUTE_PARAM_DESC_REPOSITORY)
    ),
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_proxy_root() {}

/// openapi github pages proxy.
#[utoipa::path(
    get,
    path = ROUTE_GITHUB_PAGES_ASSET,
    params(
        ("owner" = String, Path, description = ROUTE_PARAM_DESC_OWNER),
        ("repository" = String, Path, description = ROUTE_PARAM_DESC_REPOSITORY),
        ("path" = String, Path, description = ROUTE_PARAM_DESC_PATH)
    ),
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_github_pages_proxy() {}
