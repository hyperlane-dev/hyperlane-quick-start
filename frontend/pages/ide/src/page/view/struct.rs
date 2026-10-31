use super::*;

/// The props of the ide page.
#[derive(Clone, Copy, Debug, Default)]
pub struct IdePageProps {
    /// The state the page renders from.
    pub state: IdeState,
}

/// The props of the editor of the page.
#[derive(Clone, Copy, Debug, Default)]
pub struct IdeEditorProps {
    /// The state the page renders from.
    pub state: IdeState,
}
