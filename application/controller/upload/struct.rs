use super::*;

/// Route handler for registering a new chunked file upload session.
#[route("/api/upload/register")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RegisterRoute;

/// Route handler for saving a single file chunk during upload.
#[route("/api/upload/save")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct SaveRoute;

/// Route handler for merging all uploaded chunks into the final file.
#[route("/api/upload/merge")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct MergeRoute;
