use super::*;

/// Implementation of methods for `WebSocketService`.
impl WebSocketService {
    /// Serializes a `WebSocketMessage` into a JSON response body with a timestamp.
    ///
    /// # Arguments
    ///
    /// - `&WebSocketMessage`: The message to serialize.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The JSON string of the response, or an error if serialization fails or the message is invalid.
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
