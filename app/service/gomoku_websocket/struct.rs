use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuConnectedHook;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuRequestHook;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuSendedHook;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuClosedHook;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuWebSocketService;
