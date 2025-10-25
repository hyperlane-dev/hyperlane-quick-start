use super::*;

impl Default for ChatSession {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            messages: Vec::new(),
            last_activity: Instant::now(),
        }
    }
}

impl ChatSession {
    pub fn add_message(&mut self, role: String, content: String) {
        let mut message: ChatMessage = ChatMessage::default();
        message.set_role(role).set_content(content);
        self.get_mut_messages().push(message);
        self.set_last_activity(std::time::Instant::now());
        if self.get_messages().len() > 20 {
            let len: usize = self.get_messages().len();
            self.get_mut_messages().drain(0..len - 20);
        }
    }
}
