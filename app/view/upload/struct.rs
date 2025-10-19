use super::*;

#[route("/upload")]
pub struct UploadViewRoute;

#[route("/upload/file/{upload_dir}/{upload_file}")]
pub struct UploadFileRoute;
