use super::*;

/// The kind of a segment of a diff.
///
/// A segment is either the same in both texts, only in the modified text or
/// only in the original text.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DiffKind {
    /// The segment is the same in both texts.
    #[default]
    Unchanged,
    /// The segment is only in the modified text.
    Added,
    /// The segment is only in the original text.
    Removed,
}

/// The kind of a row of the diff result.
///
/// A row holds one line of the original text and one line of the modified
/// text, so the two panes of the split view stay aligned.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DiffRowKind {
    /// The line is the same in both texts.
    #[default]
    Unchanged,
    /// The line changed, both sides carry their own segments.
    Changed,
    /// The line was added to the modified text.
    Added,
    /// The line was removed from the original text.
    Removed,
}

/// The side of the result a cell of the split view shows.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DiffSide {
    /// The side that holds the original text.
    #[default]
    Original,
    /// The side that holds the modified text.
    Modified,
}
