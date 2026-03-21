use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct MessageResponse {
    pub(super) message: String,
    #[get(type(copy))]
    pub(super) time: i64,
}
