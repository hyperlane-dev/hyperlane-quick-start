use super::*;

/// openapi blog post create.
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

/// openapi blog post update.
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

/// openapi blog post delete.
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

/// openapi blog post get.
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

/// openapi blog post list.
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

/// openapi blog post my list.
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

/// openapi blog post like.
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

/// openapi blog post favorite.
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

/// openapi blog post favorite list.
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

/// openapi blog comment create.
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

/// openapi blog comment delete.
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

/// openapi blog comment list.
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

/// openapi blog image upload.
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

/// openapi blog image download.
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
