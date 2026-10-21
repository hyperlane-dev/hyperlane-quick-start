use super::*;

/// The reactive state the JSON editor page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct JsonState {
    /// The text of the document.
    #[get(type(copy))]
    pub input: Signal<String>,
    /// The parsed document.
    #[get(type(copy))]
    pub root: Signal<Option<Value>>,
    /// The nodes of the tree that are visible.
    #[get(type(copy))]
    pub nodes: Signal<Vec<JsonNode>>,
    /// The paths of the nodes that are open.
    #[get(type(copy))]
    pub expanded: Signal<Vec<Vec<String>>>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The message of the last success.
    #[get(type(copy))]
    pub notice: Signal<String>,
    /// The path of the node that is edited.
    #[get(type(copy))]
    pub edit_path: Signal<Vec<String>>,
    /// The key of the node that is edited.
    #[get(type(copy))]
    pub edit_key: Signal<String>,
    /// The kind of the node that is edited.
    #[get(type(copy))]
    pub edit_type: Signal<String>,
    /// The value of the node that is edited.
    #[get(type(copy))]
    pub edit_value: Signal<String>,
}
