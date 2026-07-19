use super::*;

/// Hook invoked when a chat WebSocket connection is established.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatConnectedHook;

/// Hook invoked when a chat WebSocket request is received.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatRequestHook;

/// Hook invoked after a chat message has been sent.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatSendedHook;

/// Hook invoked when a chat WebSocket connection is closed.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatClosedHook;

/// Service for handling chat WebSocket connections, GPT message processing, and chat history.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatService;
