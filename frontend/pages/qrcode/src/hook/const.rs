/// The key the list of the QR codes is kept under in the browser storage.
pub(crate) const STORAGE_KEY: &str = "qrcode";

/// The name of the file the list of the QR codes is written to.
pub(crate) const EXPORT_FILE_NAME: &str = "qrcode.json";

/// The value of the low error correction level.
pub(crate) const LEVEL_LOW: &str = "low";

/// The value of the medium error correction level.
pub(crate) const LEVEL_MEDIUM: &str = "medium";

/// The value of the quartile error correction level.
pub(crate) const LEVEL_QUARTILE: &str = "quartile";

/// The value of the high error correction level.
pub(crate) const LEVEL_HIGH: &str = "high";

/// The empty text of the page.
pub(crate) const TEXT_EMPTY: &str = "";

/// The number of pixels one module of a QR code is drawn with.
pub(crate) const MODULE_PIXELS: u32 = 8;

/// The width of an empty canvas, the size the browser gives a canvas.
pub(crate) const CANVAS_WIDTH: u32 = 300;

/// The height of an empty canvas, the size the browser gives a canvas.
pub(crate) const CANVAS_HEIGHT: u32 = 150;

/// The name of the colour of the dark modules of the theme.
pub(crate) const COLOR_DARK: &str = "--foreground";

/// The name of the colour of the light modules of the theme.
pub(crate) const COLOR_LIGHT: &str = "--background";

/// The colour of the dark modules when the theme holds no colour.
pub(crate) const FALLBACK_DARK: &str = "#000000";

/// The colour of the light modules when the theme holds no colour.
pub(crate) const FALLBACK_LIGHT: &str = "#ffffff";
