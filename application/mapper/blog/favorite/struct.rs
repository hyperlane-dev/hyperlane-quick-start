use super::*;

/// SeaORM entity model for the `blog_favorite` table, representing a user's favorite (bookmark) on a blog post.
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
#[sea_orm(table_name = "blog_favorite", schema_name = "public")]
pub struct Model {
    /// Unique primary key identifier for the favorite record.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the blog post that was favorited.
    #[get(type(copy))]
    pub(super) post_id: i32,
    /// The foreign key referencing the user who favorited the post.
    #[get(type(copy))]
    pub(super) user_id: i32,
    /// The timestamp when the favorite record was created.
    pub(super) created_at: Option<NaiveDateTime>,
}
