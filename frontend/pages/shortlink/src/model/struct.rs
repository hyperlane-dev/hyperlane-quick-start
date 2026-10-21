use super::*;

/// The body of a short link the visitor asks for.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ShortlinkInsertRequest {
    /// The long URL the short link points at.
    pub url: String,
}
