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
#[sea_orm(table_name = "order_user", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    pub(super) username: String,
    pub(super) password_hash: String,
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
    #[get(type(copy))]
    pub(super) role: i16,
    #[get(type(copy))]
    pub(super) status: i16,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}
