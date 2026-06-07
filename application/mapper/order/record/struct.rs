use super::*;

/// SeaORM entity model for the `order_record` table, representing a financial transaction record.
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
    /// Unique primary key identifier for the record.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The unique bill number generated for this transaction.
    pub(super) bill_no: String,
    /// The foreign key referencing the user who created the record.
    #[get(type(copy))]
    pub(super) user_id: i32,
    /// The monetary amount of the transaction.
    pub(super) amount: Decimal,
    /// The category classification of the transaction (e.g., "food", "transport").
    pub(super) category: String,
    /// The type of the transaction (e.g., "income", "expense").
    pub(super) transaction_type: String,
    /// The optional description or note for the transaction.
    pub(super) description: Option<String>,
    /// The date on which the transaction occurred.
    pub(super) bill_date: NaiveDate,
    /// The timestamp when the record was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the record was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}
