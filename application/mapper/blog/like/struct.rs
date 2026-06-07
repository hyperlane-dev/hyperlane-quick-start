use super::*;

/// SeaORM entity model for the `blog_like` table, representing a user's like on a blog post.
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
#[sea_orm(table_name = "blog_like", schema_name = "public")]
pub struct Model {
    /// Unique primary key identifier for the like record.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the blog post that was liked.
    #[get(type(copy))]
    pub(super) post_id: i32,
    /// The foreign key referencing the user who liked the post.
    #[get(type(copy))]
    pub(super) user_id: i32,
    /// The timestamp when the like record was created.
    pub(super) created_at: Option<NaiveDateTime>,
}
