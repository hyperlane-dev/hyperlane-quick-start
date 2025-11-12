use super::*;

#[derive(Debug, Default, Data, Clone, ToSchema)]
pub struct UploadedFile {
    file_name: String,
    file_path: String,
    file_size: u64,
    upload_time: String,
    file_url: String,
    content_type: String,
}
