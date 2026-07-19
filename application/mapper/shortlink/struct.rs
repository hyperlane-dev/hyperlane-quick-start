use super::*;

/// SeaORM entity model for the `shortlink` table, representing a short URL mapping to an original URL.
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
#[sea_orm(table_name = "shortlink", schema_name = "public")]
pub struct Model {
    /// Unique primary key identifier used as the short link token.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The original target URL that the short link redirects to.
    pub(super) url: String,
    /// The timestamp when the short link was created.
    pub(super) created_at: Option<NaiveDateTime>,
}
