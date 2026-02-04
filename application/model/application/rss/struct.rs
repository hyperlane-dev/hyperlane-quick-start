use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UploadedFile {
    #[get(pub)]
    pub(super) file_name: String,
    #[get(pub)]
    pub(super) file_path: String,
    #[get(type(copy), pub)]
    pub(super) file_size: u64,
    #[get(pub)]
    pub(super) upload_time: String,
    #[get(pub)]
    pub(super) file_url: String,
    #[get(pub)]
    pub(super) content_type: String,
}
