use super::*;

/// One link the not found card offers.
///
/// The card links back to the repository of the server and to the home page
/// of the server, so a visitor that landed on a missing page always has a
/// way out.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NotFoundLink {
    /// The label of the link.
    pub label: String,
    /// The address the link opens.
    pub url: String,
    /// The target the link opens in, empty for the current tab.
    pub target: String,
}
