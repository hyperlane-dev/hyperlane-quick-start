use super::*;

/// Marker struct for the tracking record SeaORM entity mapping.
pub struct TrackingMapper;

/// SeaORM entity model for the `tracking_record` table, representing an HTTP request tracking entry.
#[derive(Clone, Data, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "tracking_record")]
pub struct Model {
    /// Unique primary key identifier for the tracking record.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i64,
    /// The serialized HTTP headers of the tracked request.
    pub(super) headers: String,
    /// The serialized HTTP body content of the tracked request.
    pub(super) body: String,
    /// The Unix timestamp (in milliseconds) when the request was received.
    #[get(type(copy))]
    pub(super) timestamp: i64,
    /// The timestamp when the tracking record was created.
    pub(super) created_at: Option<NaiveDateTime>,
}
