use super::*;

/// Route structure for the upload view endpoints.
#[route("/upload")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UploadViewRoute;

/// Route structure for the file upload endpoint.
#[route("/upload/file/{upload_dir}/{upload_file}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UploadFileRoute;
