/// Maximum list limit.
pub const MAX_LIST_LIMIT: u64 = 100;

/// Error message when invalid post id.
pub const ERROR_INVALID_POST_ID: &str = "Invalid post ID";

/// Error message when post id required.
pub const ERROR_POST_ID_REQUIRED: &str = "Post ID is required";

/// Error message when post not found.
pub const ERROR_POST_NOT_FOUND: &str = "Post not found";

/// Success post deleted.
pub const SUCCESS_POST_DELETED: &str = "Post deleted successfully";

/// Error message when invalid comment id.
pub const ERROR_INVALID_COMMENT_ID: &str = "Invalid comment ID";

/// Error message when comment id required.
pub const ERROR_COMMENT_ID_REQUIRED: &str = "Comment ID is required";

/// Success comment deleted.
pub const SUCCESS_COMMENT_DELETED: &str = "Comment deleted successfully";

/// Error message when invalid image id.
pub const ERROR_INVALID_IMAGE_ID: &str = "Invalid image ID";

/// Error message when image id required.
pub const ERROR_IMAGE_ID_REQUIRED: &str = "Image ID is required";

/// Error message when image not found.
pub const ERROR_IMAGE_NOT_FOUND: &str = "Image not found";

/// Error message when missing x file name.
pub const ERROR_MISSING_X_FILE_NAME: &str = "Missing X-File-Name header";

/// Error message when missing x mime type.
pub const ERROR_MISSING_X_MIME_TYPE: &str = "Missing X-Mime-Type header";

/// Error message when post id required query.
pub const ERROR_POST_ID_REQUIRED_QUERY: &str = "Post ID is required";

/// OpenAPI response description for bad request.
pub const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad request";

/// OpenAPI response description for blog favorite list retrieved.
pub const OPENAPI_DESCRIPTION_BLOG_FAVORITE_LIST_RETRIEVED: &str =
    "Favorite post list retrieved successfully";

/// OpenAPI response description for blog favorite toggled.
pub const OPENAPI_DESCRIPTION_BLOG_FAVORITE_TOGGLED: &str = "Favorite toggled successfully";

/// OpenAPI response description for blog image downloaded.
pub const OPENAPI_DESCRIPTION_BLOG_IMAGE_DOWNLOADED: &str = "Image downloaded successfully";

/// OpenAPI response description for blog image uploaded.
pub const OPENAPI_DESCRIPTION_BLOG_IMAGE_UPLOADED: &str = "Image uploaded successfully";

/// OpenAPI response description for blog like toggled.
pub const OPENAPI_DESCRIPTION_BLOG_LIKE_TOGGLED: &str = "Like toggled successfully";

/// OpenAPI response description for blog post created.
pub const OPENAPI_DESCRIPTION_BLOG_POST_CREATED: &str = "Blog post created successfully";

/// OpenAPI response description for blog post deleted.
pub const OPENAPI_DESCRIPTION_BLOG_POST_DELETED: &str = "Blog post deleted successfully";

/// OpenAPI response description for blog post list retrieved.
pub const OPENAPI_DESCRIPTION_BLOG_POST_LIST_RETRIEVED: &str =
    "Blog post list retrieved successfully";

/// OpenAPI response description for blog post my list retrieved.
pub const OPENAPI_DESCRIPTION_BLOG_POST_MY_LIST_RETRIEVED: &str =
    "My blog post list retrieved successfully";

/// OpenAPI response description for blog post retrieved.
pub const OPENAPI_DESCRIPTION_BLOG_POST_RETRIEVED: &str = "Blog post retrieved successfully";

/// OpenAPI response description for blog post updated.
pub const OPENAPI_DESCRIPTION_BLOG_POST_UPDATED: &str = "Blog post updated successfully";

/// OpenAPI response description for comment created.
pub const OPENAPI_DESCRIPTION_COMMENT_CREATED: &str = "Comment created successfully";

/// OpenAPI response description for comment deleted.
pub const OPENAPI_DESCRIPTION_COMMENT_DELETED: &str = "Comment deleted successfully";

/// OpenAPI response description for comment list retrieved.
pub const OPENAPI_DESCRIPTION_COMMENT_LIST_RETRIEVED: &str = "Comment list retrieved successfully";

/// OpenAPI response description for comment not found.
pub const OPENAPI_DESCRIPTION_COMMENT_NOT_FOUND: &str = "Comment not found";

/// OpenAPI response description for internal server error.
pub const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal server error";

/// OpenAPI response description for unauthorized.
pub const OPENAPI_DESCRIPTION_UNAUTHORIZED: &str = "Unauthorized";

/// OpenAPI description of the comment id parameter.
pub const OPENAPI_PARAM_DESCRIPTION_COMMENT_ID: &str = "Comment ID";

/// OpenAPI description of the image id parameter.
pub const OPENAPI_PARAM_DESCRIPTION_IMAGE_ID: &str = "Image ID";

/// OpenAPI description of the post id parameter.
pub const OPENAPI_PARAM_DESCRIPTION_POST_ID: &str = "Post ID";

/// OpenAPI route path for the blog comment create endpoint.
pub const OPENAPI_PATH_BLOG_COMMENT_CREATE: &str = "/api/blog/comment/create";

/// OpenAPI route path for the blog comment delete endpoint.
pub const OPENAPI_PATH_BLOG_COMMENT_DELETE: &str = "/api/blog/comment/delete/{id}";

/// OpenAPI route path for the blog comment list endpoint.
pub const OPENAPI_PATH_BLOG_COMMENT_LIST: &str = "/api/blog/comment/list";

/// OpenAPI route path for the blog image download endpoint.
pub const OPENAPI_PATH_BLOG_IMAGE_DOWNLOAD: &str = "/api/blog/image/download/{id}";

/// OpenAPI route path for the blog image upload endpoint.
pub const OPENAPI_PATH_BLOG_IMAGE_UPLOAD: &str = "/api/blog/image/upload";

/// OpenAPI route path for the blog post create endpoint.
pub const OPENAPI_PATH_BLOG_POST_CREATE: &str = "/api/blog/post/create";

/// OpenAPI route path for the blog post delete endpoint.
pub const OPENAPI_PATH_BLOG_POST_DELETE: &str = "/api/blog/post/delete/{id}";

/// OpenAPI route path for the blog post favorite endpoint.
pub const OPENAPI_PATH_BLOG_POST_FAVORITE: &str = "/api/blog/post/favorite/{id}";

/// OpenAPI route path for the blog post favorite list endpoint.
pub const OPENAPI_PATH_BLOG_POST_FAVORITE_LIST: &str = "/api/blog/post/favorite-list";

/// OpenAPI route path for the blog post get endpoint.
pub const OPENAPI_PATH_BLOG_POST_GET: &str = "/api/blog/post/get/{id}";

/// OpenAPI route path for the blog post like endpoint.
pub const OPENAPI_PATH_BLOG_POST_LIKE: &str = "/api/blog/post/like/{id}";

/// OpenAPI route path for the blog post list endpoint.
pub const OPENAPI_PATH_BLOG_POST_LIST: &str = "/api/blog/post/list";

/// OpenAPI route path for the blog post my list endpoint.
pub const OPENAPI_PATH_BLOG_POST_MY_LIST: &str = "/api/blog/post/my-list";

/// OpenAPI route path for the blog post update endpoint.
pub const OPENAPI_PATH_BLOG_POST_UPDATE: &str = "/api/blog/post/update/{id}";
