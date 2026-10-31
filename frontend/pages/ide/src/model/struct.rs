use super::*;

/// One language the scratchpad can hold.
///
/// The identifier of the language is the one the editor of the page knows,
/// the name is the one the page shows.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IdeLanguage {
    /// The identifier of the language.
    pub id: String,
    /// The name of the language.
    pub name: String,
}
