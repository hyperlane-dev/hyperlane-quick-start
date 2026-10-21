use super::*;

/// The props of the shortlink page.
#[derive(Clone, Copy, Debug, Default)]
pub struct ShortlinkPageProps {
    /// The state the page renders from.
    pub state: ShortlinkState,
}
