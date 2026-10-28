use super::*;

/// SeaORM entity model for the `notification` table, representing a user notification record.
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
#[sea_orm(table_name = "notification", schema_name = "public")]
pub struct Model {
    /// Unique primary key identifier for the notification.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The foreign key referencing the user who owns this notification.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: i32,
    /// The title of the notification.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) title: String,
    /// The body content of the notification.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// The category type of the notification (e.g., "system", "message", "alert").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) notification_type: String,
    /// Flag indicating whether the notification has been read by the user.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_read: bool,
    /// Soft delete flag indicating whether the notification has been marked as deleted.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_deleted: bool,
    /// The timestamp when the notification was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
    /// The timestamp when the notification was last updated.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<NaiveDateTime>,
}
