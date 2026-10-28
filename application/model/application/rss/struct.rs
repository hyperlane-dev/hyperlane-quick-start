use super::*;

/// Represents an uploaded file with metadata for RSS feed generation.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UploadedFile {
    /// The file name on disk.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_name: String,
    /// The relative file path within the upload directory.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_path: String,
    /// The size of the file in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_size: u64,
    /// The string-formatted upload timestamp.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) upload_time: String,
    /// The URL path for accessing the file via HTTP.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_url: String,
    /// The MIME content type of the file.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content_type: String,
}
