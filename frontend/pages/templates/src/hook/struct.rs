use super::*;

/// The reactive state the landing page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct TemplatesState {
    /// The moment the page was opened.
    #[get(type(copy))]
    pub time: Signal<String>,
    /// The links the card offers.
    #[get(type(copy))]
    pub links: Signal<Vec<TemplatesLink>>,
}
