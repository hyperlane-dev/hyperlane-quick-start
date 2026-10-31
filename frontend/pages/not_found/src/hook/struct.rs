use super::*;

/// The reactive state the not found page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct NotFoundState {
    /// The status code the page reports.
    #[get(type(copy))]
    pub code: Signal<String>,
}
