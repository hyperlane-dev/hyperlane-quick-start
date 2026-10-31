use super::*;

/// One segment of a diff.
///
/// A segment carries the text of the segment and the kind of the change it
/// stands for. The segments of a row are the words of the line, so a changed
/// line can mark the words that differ.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiffPart {
    /// The kind of the segment.
    pub kind: DiffKind,
    /// The text of the segment.
    pub value: String,
}

/// One row of the diff result.
///
/// A row holds the segments of the original text and the segments of the
/// modified text, so the split view renders both panes from one row and the
/// inline view renders the two sides one after the other.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiffRow {
    /// The kind of the row.
    pub kind: DiffRowKind,
    /// The segments of the original text.
    pub original: Vec<DiffPart>,
    /// The segments of the modified text.
    pub modified: Vec<DiffPart>,
}
