use super::*;

#[route("/hello/{name}")]
#[derive(Clone, Copy, Debug, Default)]
pub struct HelloRoute;
