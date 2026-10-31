use super::*;

/// The variant of the primary buttons of the page.
pub(crate) const VARIANT_PRIMARY: EuvButtonVariant = EuvButtonVariant::Primary;

/// The variant of the failure alerts of the page.
pub(crate) const VARIANT_ERROR: AlertVariant = AlertVariant::Error;

/// The variant of the success alerts of the page.
pub(crate) const VARIANT_SUCCESS: AlertVariant = AlertVariant::Success;

/// The icon of the page header.
pub(crate) const ICON_PAGE: &str = "📤";

/// The title of the page header.
pub(crate) const TITLE_PAGE: &str = "Hyperlane File Upload";

/// The subtitle of the page header.
pub(crate) const SUBTITLE_PAGE: &str = "Upload and manage your files";

/// The title of the upload card.
pub(crate) const TITLE_UPLOAD: &str = "Upload files";

/// The label of the file picker.
pub(crate) const LABEL_PICK: &str = "Select files";

/// The hint below the file picker.
pub(crate) const HINT_PICK: &str = "The files are uploaded in chunks of 512 KB";

/// The title of the history card.
pub(crate) const TITLE_HISTORY: &str = "Upload history";

/// The label of the export button.
pub(crate) const LABEL_EXPORT: &str = "Export data";

/// The label of the import button.
pub(crate) const LABEL_IMPORT: &str = "Import data";

/// The hint below the import button.
pub(crate) const HINT_IMPORT: &str = "Import a file that was exported from this page";

/// The title of the empty upload list.
pub(crate) const TITLE_EMPTY: &str = "No file has been picked yet";

/// The text of the empty upload list.
pub(crate) const TEXT_EMPTY_LIST: &str = "Pick one or more files to upload them.";

/// The title of the empty history list.
pub(crate) const TITLE_EMPTY_HISTORY: &str = "The upload history is empty";

/// The text of the empty history list.
pub(crate) const TEXT_EMPTY_HISTORY: &str = "The files you upload are listed here.";

/// The suffix of the kilobyte unit.
pub(crate) const UNIT_KILOBYTE: &str = "KB";

/// The suffix of the megabyte unit.
pub(crate) const UNIT_MEGABYTE: &str = "MB";

/// The suffix of the gigabyte unit.
pub(crate) const UNIT_GIGABYTE: &str = "GB";

/// The suffix of the byte unit.
pub(crate) const UNIT_BYTE: &str = "B";
