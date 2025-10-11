use super::*;

pub fn get_response_body(body: &str) -> String {
    let mut response: MessageResponse = MessageResponse::default();
    response.set_message(body.to_owned()).set_time(date());
    serde_json::to_string(&response).unwrap_or_default()
}
