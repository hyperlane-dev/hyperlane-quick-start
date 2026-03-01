use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShortlinkInsertRequest {
    pub(super) url: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShortlinkRecord {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    pub(super) url: String,
    pub(super) created_at: String,
}
