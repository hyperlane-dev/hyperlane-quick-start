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
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the user who owns this notification.
    #[get(type(copy))]
    pub(super) user_id: i32,
    /// The title of the notification.
    pub(super) title: String,
    /// The body content of the notification.
    pub(super) content: String,
    /// The category type of the notification (e.g., "system", "message", "alert").
    pub(super) notification_type: String,
    /// Flag indicating whether the notification has been read by the user.
    #[get(type(copy))]
    pub(super) is_read: bool,
    /// Soft delete flag indicating whether the notification has been marked as deleted.
    #[get(type(copy))]
    pub(super) is_deleted: bool,
    /// The timestamp when the notification was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the notification was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}
