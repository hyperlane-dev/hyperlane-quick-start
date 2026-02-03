use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct MysqlRecord {
    #[get(pub(crate))]
    pub(super) key: String,
    #[get(pub(crate))]
    pub(super) value: String,
}
