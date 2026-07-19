use super::*;

/// Service for the euv online playground — encodes project ids, manages
/// per-user project directories, and drives `wasm-pack` builds. All
/// methods are stateless so the struct is zero-sized and
/// `#[derive(Clone, Copy, Default)]` is enough to make it freely
/// shareable.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundService;
