use super::*;

impl WebSocketService {
    #[instrument_trace]
    pub fn get_response_body(body: &WebSocketMessage) -> Result<String, String> {
        if body.is_valid() {
            return Err(ERROR_INVALID_MESSAGE.to_string());
        }
        let mut response: MessageResponse = MessageResponse::default();
        response
            .set_message(body.get_message().clone())
            .set_time(Utc::now().timestamp_millis());
        serde_json::to_string(&response).map_err(|error: serde_json::Error| error.to_string())
    }
}
