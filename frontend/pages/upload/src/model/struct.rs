use super::*;

/// The flat answer the three upload endpoints answer with.
///
/// The upload endpoints do not use the envelope of the other endpoints: they
/// answer with the status code, the address of the uploaded file and the
/// message of the call in one flat object.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct UploadResponse {
    /// The business status code; `200` marks a successful call.
    pub code: i32,
    /// The address of the uploaded file.
    pub url: String,
    /// The message of the call.
    pub msg: String,
}

/// One file of the upload list of the page.
#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub struct UploadEntry {
    /// The identifier of the upload session of the file.
    pub id: String,
    /// The name of the file.
    pub name: String,
    /// The size of the file in bytes.
    pub size: f64,
    /// The share of the file that has been uploaded.
    pub progress: f64,
    /// The state of the upload of the file.
    pub status: UploadStatus,
    /// The address the uploaded file is served under.
    pub url: String,
    /// The message of the last failure of the file.
    pub error: String,
}

/// One record of the upload history of the browser.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct UploadHistoryItem {
    /// The identifier of the upload session of the file.
    pub id: String,
    /// The name of the file.
    pub name: String,
    /// The size of the file in bytes.
    pub size: f64,
    /// The share of the file that has been uploaded.
    pub progress: f64,
    /// The address the uploaded file is served under.
    pub url: String,
    /// The moment the file was uploaded.
    pub upload_time: String,
}
