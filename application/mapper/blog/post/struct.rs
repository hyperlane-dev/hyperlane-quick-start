use super::*;

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
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    #[get(type(copy))]
    pub(super) user_id: i32,
    pub(super) title: String,
    pub(super) summary: Option<String>,
    pub(super) content: String,
    #[get(type(copy))]
    pub(super) cover_image_id: i32,
    #[get(type(copy))]
    pub(super) is_published: bool,
    #[get(type(copy))]
    pub(super) is_deleted: bool,
    #[get(type(copy))]
    pub(super) view_count: i32,
    #[get(type(copy))]
    pub(super) like_count: i32,
    #[get(type(copy))]
    pub(super) favorite_count: i32,
    #[get(type(copy))]
    pub(super) comment_count: i32,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}
