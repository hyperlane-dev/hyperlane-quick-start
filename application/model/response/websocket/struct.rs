use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct MessageResponse {
    #[get(pub(crate))]
    pub(super) message: String,
    #[get(type(copy), pub(crate))]
    pub(super) time: i64,
}
