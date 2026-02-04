use super::*;

#[route("/static/{path:.*}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct StaticResourceRoute;
