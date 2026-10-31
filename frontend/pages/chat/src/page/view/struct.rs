use super::*;

/// The props of the chat page.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChatPageProps {
    /// The state the page renders from.
    pub state: ChatState,
}

/// The props of one message of the room.
#[derive(Clone, Debug, Default)]
pub struct ChatMessageProps {
    /// The state the page renders from.
    pub state: ChatState,
    /// The message the row shows.
    pub message: ChatMessage,
}

/// The props of one visitor of the mention list.
#[derive(Clone, Debug, Default)]
pub struct MentionProps {
    /// The state the page renders from.
    pub state: ChatState,
    /// The visitor the row shows.
    pub user: OnlineUser,
}
