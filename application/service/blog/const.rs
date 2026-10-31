/// Error when the blog post does not exist.
pub(crate) const ERROR_BLOG_POST_NOT_FOUND: &str = "Blog post not found";

/// Error when updating a post owned by someone else.
pub(crate) const ERROR_UPDATE_OWN_POSTS_ONLY: &str = "You can only update your own posts";

/// Error when deleting a post owned by someone else.
pub(crate) const ERROR_DELETE_OWN_POSTS_ONLY: &str = "You can only delete your own posts";

/// Error when the parent comment does not exist.
pub(crate) const ERROR_PARENT_COMMENT_NOT_FOUND: &str = "Parent comment not found";

/// Error when the comment does not exist.
pub(crate) const ERROR_COMMENT_NOT_FOUND: &str = "Comment not found";

/// Error when deleting a comment owned by someone else.
pub(crate) const ERROR_DELETE_OWN_COMMENTS_ONLY: &str = "You can only delete your own comments";

/// Display name for an unknown author.
pub(crate) const AUTHOR_UNKNOWN: &str = "Unknown";
