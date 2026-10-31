use super::*;

/// The reactive state the ide page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct IdeState {
    /// The identifier of the language of the scratchpad.
    #[get(type(copy))]
    pub language: Signal<String>,
    /// The name of the language of the scratchpad.
    #[get(type(copy))]
    pub language_name: Signal<String>,
    /// The code of the scratchpad.
    #[get(type(copy))]
    pub code: Signal<String>,
    /// The result of the last clipboard action.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) notice: Signal<String>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
}
