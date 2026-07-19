use super::*;

/// SeaORM entity model for the `blog_post` table, representing a blog post with metadata and engagement counters.
#[derive(
    Clone,
    Data,
    Debug,
    Default,
    DeriveActiveModelBehavior,
    DeriveEntityModel,
    Deserialize,
    PartialEq,
    Serialize,
)]
#[sea_orm(table_name = "blog_post", schema_name = "public")]
pub struct Model {
    /// Unique primary key identifier for the blog post.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the user who authored the post.
    #[get(type(copy))]
    pub(super) user_id: i32,
    /// The title of the blog post.
    pub(super) title: String,
    /// The optional summary or excerpt of the post content.
    pub(super) summary: Option<String>,
    /// The full textual content of the blog post.
    pub(super) content: String,
    /// The foreign key referencing the cover image for the post.
    #[get(type(copy))]
    pub(super) cover_image_id: i32,
    /// Flag indicating whether the post has been published.
    #[get(type(copy))]
    pub(super) is_published: bool,
    /// Soft delete flag indicating whether the post has been marked as deleted.
    #[get(type(copy))]
    pub(super) is_deleted: bool,
    /// The number of times the post has been viewed.
    #[get(type(copy))]
    pub(super) view_count: i32,
    /// The number of likes the post has received.
    #[get(type(copy))]
    pub(super) like_count: i32,
    /// The number of favorites (bookmarks) the post has received.
    #[get(type(copy))]
    pub(super) favorite_count: i32,
    /// The number of comments on the post.
    #[get(type(copy))]
    pub(super) comment_count: i32,
    /// The timestamp when the post was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the post was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}
