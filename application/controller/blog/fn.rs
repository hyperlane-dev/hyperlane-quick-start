use super::*;

/// openapi blog post create.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_BLOG_POST_CREATE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_POST_CREATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_create() {}

/// openapi blog post update.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_BLOG_POST_UPDATE,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_POST_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_POST_UPDATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = ERROR_POST_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_update() {}

/// openapi blog post delete.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_BLOG_POST_DELETE,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_POST_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_POST_DELETED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = ERROR_POST_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_delete() {}

/// openapi blog post get.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_BLOG_POST_GET,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_POST_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_POST_RETRIEVED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = ERROR_POST_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_get() {}

/// openapi blog post list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_BLOG_POST_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_POST_LIST_RETRIEVED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_list() {}

/// openapi blog post my list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_BLOG_POST_MY_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_POST_MY_LIST_RETRIEVED),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_my_list() {}

/// openapi blog post like.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_BLOG_POST_LIKE,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_POST_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_LIKE_TOGGLED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = ERROR_POST_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_like() {}

/// openapi blog post favorite.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_BLOG_POST_FAVORITE,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_POST_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_FAVORITE_TOGGLED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = ERROR_POST_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_favorite() {}

/// openapi blog post favorite list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_BLOG_POST_FAVORITE_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_FAVORITE_LIST_RETRIEVED),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_favorite_list() {}

/// openapi blog comment create.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_BLOG_COMMENT_CREATE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_COMMENT_CREATED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_comment_create() {}

/// openapi blog comment delete.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_BLOG_COMMENT_DELETE,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_COMMENT_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_COMMENT_DELETED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 404, description = OPENAPI_DESCRIPTION_COMMENT_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_comment_delete() {}

/// openapi blog comment list.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_BLOG_COMMENT_LIST,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_COMMENT_LIST_RETRIEVED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_comment_list() {}

/// openapi blog image upload.
#[utoipa::path(
    post,
    path = OPENAPI_PATH_BLOG_IMAGE_UPLOAD,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_IMAGE_UPLOADED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 401, description = OPENAPI_DESCRIPTION_UNAUTHORIZED),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_image_upload() {}

/// openapi blog image download.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_BLOG_IMAGE_DOWNLOAD,
    params(
        ("id" = i32, Path, description = OPENAPI_PARAM_DESCRIPTION_IMAGE_ID)
    ),
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_BLOG_IMAGE_DOWNLOADED),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = ERROR_IMAGE_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_blog_image_download() {}
