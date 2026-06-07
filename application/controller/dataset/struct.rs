use super::*;

/// Route handler for fetching dataset content.
#[route("/dataset")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DatasetRoute;
