use super::*;

/// Represents a file upload response with status code, URL, and message.
#[derive(Clone, Copy, Data, Debug, Default, Serialize, ToSchema)]
pub struct UploadResponse<'a> {
    /// The code.
    #[get(type(copy))]
    pub(super) code: i32,
    /// The url.
    #[get(type(copy))]
    pub(super) url: &'a str,
    /// The msg.
    #[get(type(copy))]
    pub(super) msg: &'a str,
}
