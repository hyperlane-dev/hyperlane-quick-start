use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShortlinkInsertRequest {
    pub url: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShortlinkRecord {
    pub id: i32,
    pub url: String,
    pub created_at: String,
}
