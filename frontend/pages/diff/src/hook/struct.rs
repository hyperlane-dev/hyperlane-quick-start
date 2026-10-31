use super::*;

/// The reactive state the diff page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default)]
pub struct DiffState {
    /// The original text.
    #[get(type(copy))]
    pub original: Signal<String>,
    /// The comparison text.
    #[get(type(copy))]
    pub modified: Signal<String>,
    /// The view the result is rendered in.
    #[get(type(copy))]
    pub view: Signal<String>,
    /// The rows of the result.
    #[get(type(copy))]
    pub rows: Signal<Vec<DiffRow>>,
    /// The number of the lines the comparison added.
    #[get(type(copy))]
    pub added: Signal<usize>,
    /// The number of the lines the comparison removed.
    #[get(type(copy))]
    pub removed: Signal<usize>,
    /// The number of the lines the comparison kept.
    #[get(type(copy))]
    pub unchanged: Signal<usize>,
    /// The notice reported after the last call.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) notice: Signal<String>,
    /// The failure reported by the last call.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) error: Signal<String>,
}
