/// The brand title shown in the navigation header.
pub(crate) const BRAND_TITLE: &str = "Hyperlane";

/// The repository the brand link opens.
pub(crate) const BRAND_URL: &str = "https://github.com/hyperlane-dev/hyperlane";

/// The label above the navigation list.
pub(crate) const NAV_SECTION_LABEL: &str = "PAGES";

/// The theme name of the dark colour scheme.
pub(crate) const THEME_DARK: &str = "dark";

/// The glyph of the mobile menu button.
pub(crate) const MENU_GLYPH: &str = "☰";

/// The glyph of the drawer close button.
pub(crate) const CLOSE_GLYPH: &str = "✕";

/// The navigation entries: the icon, the label and the static path of the page.
pub(crate) const NAV_ITEMS: &[(&str, &str, &str)] = &[
    ("🏠", "Home", "/static/templates/index.html"),
    ("📝", "Blog", "/static/blog/index.html"),
    ("👤", "User", "/static/user/index.html"),
    ("🔑", "Auth", "/static/auth/index.html"),
    ("🔔", "Notification", "/static/notification/index.html"),
    ("🧾", "Order", "/static/order/index.html"),
    ("💬", "Chat", "/static/chat/index.html"),
    ("📤", "Upload", "/static/upload/index.html"),
    ("🔗", "Shortlink", "/static/shortlink/index.html"),
    ("🔳", "QR Code", "/static/qrcode/index.html"),
    ("🧾", "JSON", "/static/json/index.html"),
    ("📊", "Monitor", "/static/monitor/index.html"),
    ("🗄️", "Redis", "/static/redis/index.html"),
    ("📈", "Tracking", "/static/tracking/index.html"),
    ("📰", "RSS", "/static/rss/index.html"),
    ("🔍", "Trace", "/static/trace/index.html"),
    ("⚙️", "CI/CD", "/static/cicd/index.html"),
    ("🆚", "Diff", "/static/diff/index.html"),
    ("⭕", "Gomoku", "/static/gomoku/index.html"),
    ("🧪", "euv Playground", "/static/euv-playground/index.html"),
    ("⌨️", "IDE", "/static/ide/index.html"),
    ("🚫", "Not Found", "/static/not_found/index.html"),
];
