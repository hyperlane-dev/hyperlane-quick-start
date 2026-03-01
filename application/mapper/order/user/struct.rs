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
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(pub(crate))]
    pub(super) username: String,
    #[get(pub(crate))]
    pub(super) password_hash: String,
    #[get(pub(crate))]
    pub(super) nickname: Option<String>,
    #[get(pub(crate))]
    pub(super) email: Option<String>,
    #[get(pub(crate))]
    pub(super) phone: Option<String>,
    #[get(pub(crate))]
    pub(super) role: String,
    #[get(pub(crate))]
    pub(super) status: String,
    #[get(pub(crate))]
    pub(super) created_at: Option<NaiveDateTime>,
    #[get(pub(crate))]
    pub(super) updated_at: Option<NaiveDateTime>,
}
