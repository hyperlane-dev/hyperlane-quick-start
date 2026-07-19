use super::*;

/// Route handler for rendering HTML template pages.
#[route("/templates")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TemplatesRoute;
