use super::*;

impl WebSocketMessage {
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty() && !self.message.trim().is_empty()
    }
}
