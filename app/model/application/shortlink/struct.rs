use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct ShortlinkRecord {
    pub id: i32,
    pub url: String,
    pub created_at: String,
}
