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
#[sea_orm(table_name = "account_booking_record", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(pub(crate))]
    pub(super) bill_no: String,
    #[get(type(copy), pub(crate))]
    pub(super) user_id: i32,
    #[get(pub(crate))]
    pub(super) amount: Decimal,
    #[get(pub(crate))]
    pub(super) category: String,
    #[get(pub(crate))]
    pub(super) transaction_type: String,
    #[get(pub(crate))]
    pub(super) description: Option<String>,
    #[get(pub(crate))]
    pub(super) bill_date: NaiveDate,
    #[get(pub(crate))]
    pub(super) created_at: Option<NaiveDateTime>,
    #[get(pub(crate))]
    pub(super) updated_at: Option<NaiveDateTime>,
}
