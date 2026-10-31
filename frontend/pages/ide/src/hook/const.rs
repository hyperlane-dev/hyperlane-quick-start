/// The text an empty value starts with.
pub(crate) const TEXT_EMPTY: &str = "";

/// The name of the global that holds the handler of the failures of the page.
pub(crate) const GLOBAL_ERROR: &str = "__euv_ide_error";

/// The event the browser fires when a script of the page fails.
pub(crate) const EVENT_ERROR: &str = "error";

/// The event the browser fires when a promise of the page is rejected.
pub(crate) const EVENT_REJECTION: &str = "unhandledrejection";
