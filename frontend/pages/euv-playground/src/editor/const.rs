/// The path of the AMD loader of the vendored Monaco editor.
pub(crate) const LOADER_SRC: &str = "/static/ide/min/vs/loader.js";

/// The path the AMD loader of the Monaco editor resolves its modules against.
pub(crate) const MONACO_PATH: &str = "/static/ide/min/vs";

/// The name of the global that holds the container of the editor.
pub(crate) const GLOBAL_CONTAINER: &str = "__euv_editor_container";

/// The name of the global that holds the language of the editor.
pub(crate) const GLOBAL_LANGUAGE: &str = "__euv_editor_language";

/// The name of the global that holds the code of the editor.
pub(crate) const GLOBAL_VALUE: &str = "__euv_editor_value";

/// The name of the global that holds the handler of the changes.
pub(crate) const GLOBAL_CHANGE: &str = "__euv_editor_change";

/// The name of the global that holds the editor of the page.
pub(crate) const GLOBAL_EDITOR: &str = "__euv_editor_instance";

/// The name of the global that holds the pending mount of the editor.
pub(crate) const GLOBAL_PENDING: &str = "__euv_editor_pending";

/// The name of the global that holds the ready flag of the editor.
pub(crate) const GLOBAL_READY: &str = "__euv_editor_ready";

/// The number of milliseconds the mount of the editor is deferred by, so the
/// container of the editor is in the document when the editor is created.
pub(crate) const DEFER_MILLIS: i32 = 0;

/// The size of the font of the editor.
pub(crate) const FONT_SIZE: f64 = 16.0;

/// The number of the spaces of one indent of the editor.
pub(crate) const TAB_SIZE: u32 = 4;

/// The event the loader of the editor fires when it is loaded.
pub(crate) const EVENT_LOAD: &str = "load";
