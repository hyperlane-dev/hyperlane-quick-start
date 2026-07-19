use super::*;

/// Route structure for the gomoku game view endpoints.
#[route("/gomoku")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuViewRoute;
