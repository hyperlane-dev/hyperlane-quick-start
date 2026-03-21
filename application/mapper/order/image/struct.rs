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
    #[get(type(copy))]
    pub(super) id: i32,
    #[get(type(copy))]
    pub(super) record_id: i32,
    #[get(type(copy))]
    pub(super) user_id: i32,
    pub(super) file_name: String,
    pub(super) original_name: Option<String>,
    pub(super) mime_type: String,
    #[get(type(copy))]
    pub(super) file_size: i32,
    pub(super) file_data: Vec<u8>,
    pub(super) created_at: Option<NaiveDateTime>,
}
