use super::*;

/// The variant of the primary buttons of the page.
pub(crate) const VARIANT_PRIMARY: EuvButtonVariant = EuvButtonVariant::Primary;

/// The variant of the secondary buttons of the page.
pub(crate) const VARIANT_OUTLINE: EuvButtonVariant = EuvButtonVariant::Outline;

/// The variant of the failure alerts of the page.
pub(crate) const VARIANT_ERROR: AlertVariant = AlertVariant::Error;

/// The icon of the page header.
pub(crate) const ICON_PAGE: &str = "💬";

/// The title of the page header.
pub(crate) const TITLE_PAGE: &str = "Hyperlane Chat";

/// The subtitle of the page header.
pub(crate) const SUBTITLE_PAGE: &str = "Talk with the other visitors of the room";

/// The title of the room card.
pub(crate) const TITLE_ROOM: &str = "Room";

/// The label of the connected state.
pub(crate) const LABEL_CONNECTED: &str = "Connected";

/// The label of the disconnected state.
pub(crate) const LABEL_DISCONNECTED: &str = "Offline";

/// The label of the online counter.
pub(crate) const LABEL_ONLINE: &str = "visitors online";

/// The placeholder of the message field.
pub(crate) const PLACEHOLDER_MESSAGE: &str = "Write a message, use @name to mention a visitor";

/// The number of rows of the message field.
pub(crate) const ROWS_MESSAGE: i32 = 3;

/// The label of the send button.
pub(crate) const LABEL_SEND: &str = "Send";

/// The label of the file picker.
pub(crate) const LABEL_ATTACH: &str = "Attach a file";

/// The title of the mention list.
pub(crate) const TITLE_MENTION: &str = "Mention a visitor";

/// The title of the empty room.
pub(crate) const TITLE_EMPTY: &str = "The room is quiet";

/// The text of the empty room.
pub(crate) const TEXT_EMPTY_ROOM: &str = "No message has been sent yet.";

/// The text of the empty mention list.
pub(crate) const TEXT_EMPTY_MENTION: &str = "No visitor matches the mention.";

/// The text of the upload in flight.
pub(crate) const TEXT_UPLOADING: &str = "The file is being uploaded";

/// The label of the visitor of the page.
pub(crate) const LABEL_YOU: &str = "You";

/// The key that sends the message being written.
pub(crate) const KEY_ENTER: &str = "Enter";
