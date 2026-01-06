use super::*;

pub struct TrackingMapper;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tracking_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub socket_addr: String,
    pub headers: String,
    pub body: String,
    pub timestamp: i64,
    pub created_at: Option<NaiveDateTime>,
}
