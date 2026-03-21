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
#[sea_orm(table_name = "order_record", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    pub(super) bill_no: String,
    #[get(type(copy))]
    pub(super) user_id: i32,
    pub(super) amount: Decimal,
    pub(super) category: String,
    pub(super) transaction_type: String,
    pub(super) description: Option<String>,
    pub(super) bill_date: NaiveDate,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}
