use super::*;

impl WebSocketMessage {
    pub fn is_valid(&self) -> bool {
        !self.get_name().trim().is_empty() && !self.get_message().trim().is_empty()
    }
}
