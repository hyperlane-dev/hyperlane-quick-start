use super::*;

/// Application-level model representing a blog post with metadata and engagement counters.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct BlogPost {
    /// The unique identifier for the blog post.
    pub(super) id: i32,
    /// The foreign key referencing the user who authored the post.
    pub(super) user_id: i32,
    /// The title of the blog post.
    pub(super) title: String,
    /// The optional summary or excerpt of the post content.
    pub(super) summary: Option<String>,
    /// The full textual content of the blog post.
    pub(super) content: String,
    /// The foreign key referencing the cover image for the post.
    pub(super) cover_image_id: i32,
    /// Flag indicating whether the post has been published.
    pub(super) is_published: bool,
    /// Soft delete flag indicating whether the post has been marked as deleted.
    pub(super) is_deleted: bool,
    /// The number of times the post has been viewed.
    pub(super) view_count: i32,
    /// The number of likes the post has received.
    pub(super) like_count: i32,
    /// The number of favorites (bookmarks) the post has received.
    pub(super) favorite_count: i32,
    /// The number of comments on the post.
    pub(super) comment_count: i32,
    /// The timestamp when the post was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the post was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}

/// Application-level model representing a comment on a blog post with nested reply support.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct BlogComment {
    /// The unique identifier for the comment.
    pub(super) id: i32,
    /// The foreign key referencing the blog post this comment belongs to.
    pub(super) post_id: i32,
    /// The foreign key referencing the user who authored the comment.
    pub(super) user_id: i32,
    /// The foreign key referencing the parent comment for nested replies (0 if top-level).
    pub(super) parent_id: i32,
    /// The textual content of the comment.
    pub(super) content: String,
    /// Soft delete flag indicating whether the comment has been marked as deleted.
    pub(super) is_deleted: bool,
    /// The timestamp when the comment was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the comment was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}
