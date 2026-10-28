/// The view that renders the result in two panes.
pub(crate) const VIEW_SPLIT: &str = "split";

/// The view that renders the result in one pane.
pub(crate) const VIEW_INLINE: &str = "inline";

/// The text an empty input starts with.
pub(crate) const TEXT_EMPTY: &str = "";

/// The original text the example button writes into the page.
pub(crate) const EXAMPLE_ORIGINAL: &str = "The quick brown fox jumps over the lazy dog.\nThe second line is the same on both sides.\nThe third line carries the change.";

/// The comparison text the example button writes into the page.
pub(crate) const EXAMPLE_MODIFIED: &str = "The quick brown fox leaps over the lazy dog.\nThe second line is the same on both sides.\nThe third line carries the change.";

/// The notice reported after a text has been copied into the clipboard.
pub(crate) const NOTICE_COPIED: &str = "The text has been copied into the clipboard";

/// The failure reported when the clipboard cannot be written into.
pub(crate) const ERROR_CLIPBOARD: &str = "The clipboard could not be written into";

/// The failure reported when there is no text to copy.
pub(crate) const ERROR_EMPTY_TEXT: &str = "Enter text before copying";
