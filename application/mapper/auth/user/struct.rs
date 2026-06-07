use super::*;

/// SeaORM entity model for the `auth_user` table, representing an authenticated user record.
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
#[sea_orm(table_name = "auth_user", schema_name = "public")]
pub struct Model {
    /// Unique primary key identifier for the user.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The login username of the user.
    pub(super) username: String,
    /// The bcrypt-hashed password string.
    pub(super) password_hash: String,
    /// The optional email address of the user.
    pub(super) email: Option<String>,
    /// The optional phone number of the user.
    pub(super) phone: Option<String>,
    /// The numeric role identifier (e.g., admin, regular user).
    #[get(type(copy))]
    pub(super) role: i16,
    /// The numeric status indicator (e.g., active, disabled).
    #[get(type(copy))]
    pub(super) status: i16,
    /// The timestamp when the user record was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the user record was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}
