/// File extensions for text-based content that should include `charset=utf-8`
/// in their `Content-Type` header.
///
/// Only text-based formats that are human-readable or may contain character
/// encoding information are included. Binary formats (images, fonts, media, etc.)
/// are excluded since adding charset would be meaningless or could break rendering.
pub const TEXT_CONTENT_EXTENSIONS: &[&str] = &[
    "html", "htm", "css", "js", "mjs", "cjs", "json", "xml", "svg", "txt", "md", "csv", "ics",
    "map", "scss", "less", "sass", "yaml", "yml", "toml", "ini", "conf", "ts", "tsx", "jsx", "rtf",
    "log", "sh", "bat", "ps1",
];

/// File extensions that typically require HTTP Range request support for streaming.
///
/// Video and audio formats where browsers send Range requests for seeking/buffering.
pub const STREAMABLE_EXTENSIONS: &[&str] = &[
    "mp4", "webm", "ogg", "m4v", "avi", "mov", "wmv", "flv", "mkv", "mp3", "wav", "flac", "m4a",
    "aac", "oga", "pdf",
];
