use super::*;

/// SeaORM entity model for the `blog_comment` table, representing a comment on a blog post.
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
#[sea_orm(table_name = "blog_comment", schema_name = "public")]
pub struct Model {
    /// Unique primary key identifier for the comment.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the blog post this comment belongs to.
    #[get(type(copy))]
    pub(super) post_id: i32,
    /// The foreign key referencing the user who authored the comment.
    #[get(type(copy))]
    pub(super) user_id: i32,
    /// The foreign key referencing the parent comment for nested replies (0 if top-level).
    #[get(type(copy))]
    pub(super) parent_id: i32,
    /// The textual content of the comment.
    pub(super) content: String,
    /// Soft delete flag indicating whether the comment has been marked as deleted.
    #[get(type(copy))]
    pub(super) is_deleted: bool,
    /// The timestamp when the comment was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the comment was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}
