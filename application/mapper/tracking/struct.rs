use super::*;

pub struct TrackingMapper;

#[derive(Clone, Data, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "tracking_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy), pub(crate))]
    pub(super) id: i64,
    #[get(pub(crate))]
    pub(super) socket_addr: String,
    #[get(pub(crate))]
    pub(super) headers: String,
    #[get(pub(crate))]
    pub(super) body: String,
    #[get(type(copy), pub(crate))]
    pub(super) timestamp: i64,
    #[get(pub(crate))]
    pub(super) created_at: Option<NaiveDateTime>,
}
