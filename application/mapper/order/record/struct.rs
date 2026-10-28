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
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The unique bill number generated for this transaction.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bill_no: String,
    /// The foreign key referencing the user who created the record.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: i32,
    /// The monetary amount of the transaction.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) amount: Decimal,
    /// The category classification of the transaction (e.g., "food", "transport").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) category: String,
    /// The type of the transaction (e.g., "income", "expense").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) transaction_type: String,
    /// The optional description or note for the transaction.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) description: Option<String>,
    /// The date on which the transaction occurred.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bill_date: NaiveDate,
    /// The timestamp when the record was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
    /// The timestamp when the record was last updated.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<NaiveDateTime>,
}
