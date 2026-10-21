use super::*;

/// The props of the gomoku page.
#[derive(Clone, Copy, Debug, Default)]
pub struct GomokuPageProps {
    /// The state the page renders from.
    pub state: GomokuState,
}
