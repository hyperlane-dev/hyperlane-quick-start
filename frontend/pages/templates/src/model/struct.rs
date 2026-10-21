use super::*;

/// One link the landing card offers.
///
/// The card links to the repository of the server and to the home page of
/// the server, so a visitor that landed on the landing page always has a way
/// into the server.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TemplatesLink {
    /// The label of the link.
    pub label: String,
    /// The address the link opens.
    pub url: String,
    /// The target the link opens in, empty for the current tab.
    pub target: String,
}
