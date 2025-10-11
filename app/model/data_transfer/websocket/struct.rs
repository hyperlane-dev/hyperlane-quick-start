use super::*;

#[derive(Debug, Default, Data, Serialize, Deserialize)]
pub struct MessageResponse {
    message: String,
    time: String,
}
