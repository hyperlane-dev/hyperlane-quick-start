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
#[sea_orm(table_name = "blog_comment", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    #[get(type(copy))]
    pub(super) post_id: i32,
    #[get(type(copy))]
    pub(super) user_id: i32,
    #[get(type(copy))]
    pub(super) parent_id: i32,
    pub(super) content: String,
    #[get(type(copy))]
    pub(super) is_deleted: bool,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}
