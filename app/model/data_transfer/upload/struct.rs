use super::*;

#[derive(Debug, Serialize, Data, Clone, Copy, ToSchema)]
pub struct UploadResponse<'a> {
    pub code: i32,
    pub url: &'a str,
    pub msg: &'a str,
}
