use super::*;

/// One node of the tree of the document.
///
/// The node holds the place of the value in the document, the kind of the
/// value and a short preview of it, so the tree can be rendered without
/// walking the document again.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JsonNode {
    /// The path of the value in the document.
    pub path: Vec<String>,
    /// The key of the value in its parent.
    pub key: String,
    /// The depth of the value in the tree.
    pub depth: usize,
    /// The kind of the value.
    pub kind: String,
    /// Whether the value is a text that holds a document.
    pub json_string: bool,
    /// Whether the value holds other values.
    pub container: bool,
    /// Whether the value is open.
    pub expanded: bool,
    /// A short preview of the value.
    pub preview: String,
}
