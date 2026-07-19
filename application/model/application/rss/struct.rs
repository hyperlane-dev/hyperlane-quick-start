use super::*;

/// Represents an uploaded file with metadata for RSS feed generation.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UploadedFile {
    /// The file name on disk.
    pub(super) file_name: String,
    /// The relative file path within the upload directory.
    pub(super) file_path: String,
    /// The size of the file in bytes.
    #[get(type(copy))]
    pub(super) file_size: u64,
    /// The string-formatted upload timestamp.
    pub(super) upload_time: String,
    /// The URL path for accessing the file via HTTP.
    pub(super) file_url: String,
    /// The MIME content type of the file.
    pub(super) content_type: String,
}
