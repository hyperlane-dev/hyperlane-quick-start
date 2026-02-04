use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShortlinkInsertRequest {
    #[get(pub)]
    pub(super) url: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShortlinkRecord {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(pub)]
    pub(super) url: String,
    #[get(pub)]
    pub(super) created_at: String,
}
