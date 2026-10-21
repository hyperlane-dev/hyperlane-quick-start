use super::*;

/// The reactive state the shortlink page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct ShortlinkState {
    /// The long URL the visitor typed.
    #[get(type(copy))]
    pub url: Signal<String>,
    /// The short link the server answered with.
    #[get(type(copy))]
    pub shortlink: Signal<String>,
    /// Whether the short link is on the clipboard.
    #[get(type(copy))]
    pub copied: Signal<bool>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
}
