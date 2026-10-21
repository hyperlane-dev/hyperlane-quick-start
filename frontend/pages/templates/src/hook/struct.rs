use super::*;

/// The reactive state the landing page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct TemplatesState {
    /// The moment the page was served.
    #[get(type(copy))]
    pub time: Signal<String>,
}
