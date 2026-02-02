use super::*;

#[derive(Clone, Data, Debug, Default, ToSchema)]
pub struct UploadedFile {
    file_name: String,
    file_path: String,
    file_size: u64,
    upload_time: String,
    file_url: String,
    content_type: String,
}
