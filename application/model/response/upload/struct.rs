use super::*;

/// Represents a file upload response with status code, URL, and message.
#[derive(Clone, Copy, Data, Debug, Default, Serialize, ToSchema)]
pub struct UploadResponse<'a> {
    /// The code.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) code: i32,
    /// The url.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) url: &'a str,
    /// The msg.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) msg: &'a str,
}
