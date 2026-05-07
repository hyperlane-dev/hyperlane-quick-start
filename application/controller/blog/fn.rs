use super::*;

#[utoipa::path(
    post,
    path = "/api/blog/post/create",
    responses(
        (status = 200, description = "Blog post created successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_create() {}

#[utoipa::path(
    post,
    path = "/api/blog/post/update/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Blog post updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_update() {}

#[utoipa::path(
    post,
    path = "/api/blog/post/delete/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Blog post deleted successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_delete() {}

#[utoipa::path(
    get,
    path = "/api/blog/post/get/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Blog post retrieved successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_get() {}

#[utoipa::path(
    get,
    path = "/api/blog/post/list",
    responses(
        (status = 200, description = "Blog post list retrieved successfully"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_list() {}

#[utoipa::path(
    get,
    path = "/api/blog/post/my-list",
    responses(
        (status = 200, description = "My blog post list retrieved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_my_list() {}

#[utoipa::path(
    post,
    path = "/api/blog/post/like/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Like toggled successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_like() {}

#[utoipa::path(
    post,
    path = "/api/blog/post/favorite/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Favorite toggled successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_favorite() {}

#[utoipa::path(
    get,
    path = "/api/blog/post/favorite-list",
    responses(
        (status = 200, description = "Favorite post list retrieved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_post_favorite_list() {}

#[utoipa::path(
    post,
    path = "/api/blog/comment/create",
    responses(
        (status = 200, description = "Comment created successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_comment_create() {}

#[utoipa::path(
    post,
    path = "/api/blog/comment/delete/{id}",
    params(
        ("id" = i32, Path, description = "Comment ID")
    ),
    responses(
        (status = 200, description = "Comment deleted successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Comment not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_comment_delete() {}

#[utoipa::path(
    get,
    path = "/api/blog/comment/list",
    responses(
        (status = 200, description = "Comment list retrieved successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_comment_list() {}

#[utoipa::path(
    post,
    path = "/api/blog/image/upload",
    responses(
        (status = 200, description = "Image uploaded successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_image_upload() {}

#[utoipa::path(
    get,
    path = "/api/blog/image/download/{id}",
    params(
        ("id" = i32, Path, description = "Image ID")
    ),
    responses(
        (status = 200, description = "Image downloaded successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Image not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_blog_image_download() {}
