/// Default Cache-Control header value for short-term caching of dynamic content.
pub const DEFAULT_CACHE_CONTROL_SHORT_TERM: &str = "public, max-age=3600";

/// Default Cache-Control header value for static assets with long-term caching and immutability.
pub(crate) const DEFAULT_CACHE_CONTROL_STATIC_ASSETS: &str = "public, max-age=31536000, immutable";

/// Default Expires header value set to a far-future date for long-term caching.
pub(crate) const DEFAULT_EXPIRES_FAR_FUTURE: &str = "Wed, 1 Apr 8888 00:00:00 GMT";
