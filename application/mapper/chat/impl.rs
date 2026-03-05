use super::*;

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for ChatHistory {
    fn from(model: Model) -> Self {
        let mut history = ChatHistory::default();
        history
            .set_id(model.id)
            .set_session_id(model.session_id)
            .set_sender_name(model.sender_name)
            .set_sender_type(model.sender_type)
            .set_message_type(model.message_type)
            .set_content(model.content)
            .set_created_at(
                model
                    .created_at
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_default(),
            );
        history
    }
}
