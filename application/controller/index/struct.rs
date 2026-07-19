use super::*;

/// Route handler for the application root index page.
#[route("/")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct IndexRoute;
