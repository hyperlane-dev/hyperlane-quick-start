use super::*;

#[derive(Clone, Copy, Data, Debug, Serialize, ToSchema)]
pub struct UploadResponse<'a> {
    pub code: i32,
    pub url: &'a str,
    pub msg: &'a str,
}
