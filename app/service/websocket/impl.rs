use super::*;

impl WebSocketService {
    pub fn get_response_body(body: &WebSocketMessage) -> Result<String, String> {
        if body.is_valid() {
            return Err("Invalid message".to_string());
        }
        let mut response: MessageResponse = MessageResponse::default();
        response
            .set_message(body.get_message().clone())
            .set_time(date());
        serde_json::to_string(&response).map_err(|error| error.to_string())
    }
}
