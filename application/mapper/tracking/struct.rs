use super::*;

pub struct TrackingMapper;

#[derive(Clone, Data, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "tracking_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i64,
    pub(super) headers: String,
    pub(super) body: String,
    #[get(type(copy))]
    pub(super) timestamp: i64,
    pub(super) created_at: Option<NaiveDateTime>,
}
