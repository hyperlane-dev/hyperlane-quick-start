use super::*;

/// The props of the euv playground page.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlaygroundPageProps {
    /// The state the page renders from.
    pub state: PlaygroundState,
}

/// The props of the editor of the page.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlaygroundEditorProps {
    /// The state the page renders from.
    pub state: PlaygroundState,
}
