use super::*;

/// The variant of the primary buttons of the page.
pub(crate) const VARIANT_PRIMARY: EuvButtonVariant = EuvButtonVariant::Primary;

/// The variant of the secondary buttons of the page.
pub(crate) const VARIANT_OUTLINE: EuvButtonVariant = EuvButtonVariant::Outline;

/// The variant of the failure alerts of the page.
pub(crate) const VARIANT_ERROR: AlertVariant = AlertVariant::Error;

/// The variant of the success alerts of the page.
pub(crate) const VARIANT_SUCCESS: AlertVariant = AlertVariant::Success;

/// The label of the low error correction level.
pub(crate) const LABEL_LEVEL_LOW: &str = "Low";

/// The label of the medium error correction level.
pub(crate) const LABEL_LEVEL_MEDIUM: &str = "Medium";

/// The label of the quartile error correction level.
pub(crate) const LABEL_LEVEL_QUARTILE: &str = "Quartile";

/// The label of the high error correction level.
pub(crate) const LABEL_LEVEL_HIGH: &str = "High";

/// The label of the button that opens the menu of the levels.
pub(crate) const LABEL_LEVEL: &str = "Error correction";

/// The entries of the menu of the error correction levels.
pub(crate) const LEVEL_ITEMS: &[EuvDropdownItem] = &[
    EuvDropdownItem {
        label: LABEL_LEVEL_LOW,
        value: LEVEL_LOW,
        active: false,
    },
    EuvDropdownItem {
        label: LABEL_LEVEL_MEDIUM,
        value: LEVEL_MEDIUM,
        active: true,
    },
    EuvDropdownItem {
        label: LABEL_LEVEL_QUARTILE,
        value: LEVEL_QUARTILE,
        active: false,
    },
    EuvDropdownItem {
        label: LABEL_LEVEL_HIGH,
        value: LEVEL_HIGH,
        active: false,
    },
];

/// The icon of the page header.
pub(crate) const ICON_PAGE: &str = "🔳";

/// The title of the page header.
pub(crate) const TITLE_PAGE: &str = "QR Code Generator";

/// The subtitle of the page header.
pub(crate) const SUBTITLE_PAGE: &str = "Generate QR codes from text or URLs";

/// The title of the list card.
pub(crate) const TITLE_LIST: &str = "Codes";

/// The label of the import button.
pub(crate) const LABEL_IMPORT: &str = "Import";

/// The label of the add button.
pub(crate) const LABEL_ADD: &str = "Add new";

/// The label of the export button.
pub(crate) const LABEL_EXPORT: &str = "Export";

/// The label of the clear button.
pub(crate) const LABEL_CLEAR: &str = "Clear all";

/// The placeholder of the text field.
pub(crate) const PLACEHOLDER_TEXT: &str = "Enter the URL or the text to encode";

/// The number of rows of the text field.
pub(crate) const ROWS_TEXT: &str = "2";

/// The label of the remove button.
pub(crate) const LABEL_REMOVE: &str = "Remove";

/// The label of the open button.
pub(crate) const LABEL_OPEN: &str = "Visit the link";

/// The title of the empty list.
pub(crate) const TITLE_EMPTY: &str = "No code yet";

/// The text of the empty list.
pub(crate) const TEXT_EMPTY_LIST: &str = "Add a text to encode it as a QR code.";
