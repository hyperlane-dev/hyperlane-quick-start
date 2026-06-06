use super::*;

/// Route handler for serving the favicon, redirecting to the configured logo image URL.
#[route("/favicon.ico")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct FaviconRoute;
