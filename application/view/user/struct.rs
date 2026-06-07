use super::*;

/// Route structure for the user management view endpoints.
#[route("/user")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserViewRoute;
