use super::*;

/// Implementation of methods for `WebSocketMessage`.
impl WebSocketMessage {
    /// Checks whether the message is valid by ensuring both name and message fields are non-empty.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the message has a valid name and content, `false` otherwise.
    #[instrument_trace]
    pub fn is_valid(&self) -> bool {
        !self.get_name().trim().is_empty() && !self.get_message().trim().is_empty()
    }
}
