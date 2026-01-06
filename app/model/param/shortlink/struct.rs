use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Data)]
pub struct ShortlinkInsertRequest {
    pub url: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Data)]
pub struct ShortlinkRecord {
    pub id: i32,
    pub url: String,
    pub created_at: String,
}
