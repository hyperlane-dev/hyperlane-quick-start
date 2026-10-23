use super::*;

thread_local! {
    /// The connection of the chat room of the page.
    pub(crate) static CHAT_CONNECTION: RefCell<Option<ChatConnection>> = const { RefCell::new(None) };
}
