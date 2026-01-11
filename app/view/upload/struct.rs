use super::*;

#[route("/upload")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UploadViewRoute;

#[route("/upload/file/{upload_dir}/{upload_file}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UploadFileRoute;
