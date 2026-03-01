use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UploadedFile {
    pub(super) file_name: String,
    pub(super) file_path: String,
    #[get(type(copy), pub)]
    pub(super) file_size: u64,
    pub(super) upload_time: String,
    pub(super) file_url: String,
    pub(super) content_type: String,
}
