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
#[sea_orm(table_name = "order_record_image", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(type(copy), pub(crate))]
    pub(super) record_id: i32,
    #[get(type(copy), pub(crate))]
    pub(super) user_id: i32,
    #[get(pub(crate))]
    pub(super) file_name: String,
    #[get(pub(crate))]
    pub(super) original_name: Option<String>,
    #[get(pub(crate))]
    pub(super) mime_type: String,
    #[get(type(copy), pub(crate))]
    pub(super) file_size: i32,
    #[get(pub(crate))]
    pub(super) file_data: Vec<u8>,
    #[get(pub(crate))]
    pub(super) created_at: Option<NaiveDateTime>,
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined")
    }
}
