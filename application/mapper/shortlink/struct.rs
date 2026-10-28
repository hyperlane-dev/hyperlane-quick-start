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
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The original target URL that the short link redirects to.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) url: String,
    /// The timestamp when the short link was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
}
