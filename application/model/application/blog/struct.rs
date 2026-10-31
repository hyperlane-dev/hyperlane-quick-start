use super::*;

/// Application-level model representing a blog post with metadata and engagement counters.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct BlogPost {
    /// The unique identifier for the blog post.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The foreign key referencing the user who authored the post.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: i32,
    /// The title of the blog post.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) title: String,
    /// The optional summary or excerpt of the post content.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) summary: Option<String>,
    /// The full textual content of the blog post.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// The foreign key referencing the cover image for the post.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cover_image_id: i32,
    /// Flag indicating whether the post has been published.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_published: bool,
    /// Soft delete flag indicating whether the post has been marked as deleted.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_deleted: bool,
    /// The number of times the post has been viewed.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) view_count: i32,
    /// The number of likes the post has received.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) like_count: i32,
    /// The number of favorites (bookmarks) the post has received.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) favorite_count: i32,
    /// The number of comments on the post.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) comment_count: i32,
    /// The timestamp when the post was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
    /// The timestamp when the post was last updated.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<NaiveDateTime>,
}

/// Application-level model representing a comment on a blog post with nested reply support.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct BlogComment {
    /// The unique identifier for the comment.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The foreign key referencing the blog post this comment belongs to.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) post_id: i32,
    /// The foreign key referencing the user who authored the comment.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: i32,
    /// The foreign key referencing the parent comment for nested replies (0 if top-level).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) parent_id: i32,
    /// The textual content of the comment.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// Soft delete flag indicating whether the comment has been marked as deleted.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_deleted: bool,
    /// The timestamp when the comment was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
    /// The timestamp when the comment was last updated.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<NaiveDateTime>,
}
