use super::*;

impl Default for ChatSession {
    #[instrument_trace]
    fn default() -> Self {
        Self {
            session_id: String::new(),
            messages: vec![],
            last_activity: Instant::now(),
        }
    }
}

impl ChatSession {
    #[instrument_trace]
    pub fn add_message<R, C>(&mut self, role: R, content: C)
    where
        R: AsRef<str>,
        C: AsRef<str>,
    {
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
