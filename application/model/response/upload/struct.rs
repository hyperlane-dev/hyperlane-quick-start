use super::*;

#[derive(Clone, Copy, Data, Debug, Default, Serialize, ToSchema)]
pub struct UploadResponse<'a> {
    #[get(type(copy))]
    pub(super) code: i32,
    #[get(type(copy))]
    pub(super) url: &'a str,
    #[get(type(copy))]
    pub(super) msg: &'a str,
}
