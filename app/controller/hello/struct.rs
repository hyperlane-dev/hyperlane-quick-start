use super::*;

#[route("/hello/{name}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct HelloRoute;
