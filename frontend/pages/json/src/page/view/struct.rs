use super::*;

/// The props of the JSON editor page.
#[derive(Clone, Copy, Debug, Default)]
pub struct JsonPageProps {
    /// The state the page renders from.
    pub state: JsonState,
}

/// The props of one row of the tree.
#[derive(Clone, Debug, Default)]
pub struct JsonRowProps {
    /// The state the page renders from.
    pub state: JsonState,
    /// The node the row shows.
    pub node: JsonNode,
}

/// The props of the editor of one node.
#[derive(Clone, Copy, Debug, Default)]
pub struct JsonEditorProps {
    /// The state the page renders from.
    pub state: JsonState,
}
