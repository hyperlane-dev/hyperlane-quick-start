use super::*;

/// static resource route.
#[route("/static/{path:.*}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct StaticResourceRoute;
