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
#[sea_orm(table_name = "notification", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    #[get(type(copy))]
    pub(super) user_id: i32,
    pub(super) title: String,
    pub(super) content: String,
    pub(super) notification_type: String,
    #[get(type(copy))]
    pub(super) is_read: bool,
    #[get(type(copy))]
    pub(super) is_deleted: bool,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}
