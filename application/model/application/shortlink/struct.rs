use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct ShortlinkRecord {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    pub(super) url: String,
    pub(super) created_at: String,
}
