use super::*;

#[derive(Data, Debug, Default, Deserialize, Serialize)]
pub struct MessageResponse {
    message: String,
    time: String,
}
