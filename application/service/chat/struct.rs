use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatConnectedHook;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatRequestHook;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatSendedHook;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatClosedHook;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatService;
