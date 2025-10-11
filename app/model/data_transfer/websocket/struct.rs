use super::*;

#[derive(Debug, Default, Serialize, Deserialize, Data)]
pub struct MessageResponse {
    message: String,
    time: String,
}
