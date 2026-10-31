use super::*;

/// The state of one file of the upload list.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum UploadStatus {
    /// The file waits for its upload to start.
    #[default]
    Pending,
    /// The file is being uploaded.
    Uploading,
    /// The file has been uploaded.
    Done,
    /// The upload of the file failed.
    Failed,
}
