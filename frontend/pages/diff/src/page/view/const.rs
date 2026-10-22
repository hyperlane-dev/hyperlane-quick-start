use super::*;

/// The tabs of the result.
pub(crate) const VIEW_ITEMS: &[EuvTabItem] = &[
    EuvTabItem {
        key: VIEW_SPLIT,
        label: "Split View",
    },
    EuvTabItem {
        key: VIEW_INLINE,
        label: "Inline View",
    },
];

/// The title of the card that holds the original text.
pub(crate) const TITLE_ORIGINAL: &str = "Original Text";

/// The title of the card that holds the modified text.
pub(crate) const TITLE_MODIFIED: &str = "Comparison Text";

/// The title of the card that holds the result.
pub(crate) const TITLE_RESULT: &str = "Difference";

/// The label above the original text.
pub(crate) const LABEL_ORIGINAL: &str = "Original Text";

/// The label above the modified text.
pub(crate) const LABEL_MODIFIED: &str = "Comparison Text";

/// The label of the copy button.
pub(crate) const LABEL_COPY: &str = "Copy";

/// The label of the clear button.
pub(crate) const LABEL_CLEAR: &str = "Clear";

/// The label of the added counter.
pub(crate) const LABEL_ADDED: &str = "Added lines";

/// The label of the removed counter.
pub(crate) const LABEL_REMOVED: &str = "Removed lines";

/// The label of the unchanged counter.
pub(crate) const LABEL_UNCHANGED: &str = "Unchanged lines";

/// The placeholder of the original text.
pub(crate) const PLACEHOLDER_ORIGINAL: &str = "The text the comparison starts from";

/// The placeholder of the modified text.
pub(crate) const PLACEHOLDER_MODIFIED: &str = "The text the comparison runs against";

/// The identifier of the original text.
pub(crate) const ID_ORIGINAL: &str = "diff-original";

/// The identifier of the modified text.
pub(crate) const ID_MODIFIED: &str = "diff-modified";

/// The number of the rows of the two text areas.
pub(crate) const ROWS_TEXT: &str = "10";

/// The marker of a line that was removed.
pub(crate) const MARK_REMOVED: &str = "−";

/// The marker of a line that was added.
pub(crate) const MARK_ADDED: &str = "+";

/// The marker of a line that stayed the same.
pub(crate) const MARK_UNCHANGED: &str = " ";

/// The text shown when both texts are empty.
pub(crate) const TEXT_EMPTY_RESULT: &str = "Enter text to compare";

/// The title of the empty state.
pub(crate) const TITLE_EMPTY: &str = "Nothing to compare yet";
