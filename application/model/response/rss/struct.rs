use super::*;

/// Represents an RSS channel containing feed metadata and items.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssChannel {
    /// The title.
    pub(crate) title: String,
    /// The link.
    pub(crate) link: String,
    /// The description.
    pub(crate) description: String,
    /// The language.
    pub(crate) language: String,
    /// The items.
    pub(crate) items: Vec<RssItem>,
}

/// Represents a single RSS feed item with title, link, and optional enclosure.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssItem {
    /// The title.
    pub(crate) title: String,
    /// The link.
    pub(crate) link: String,
    /// The description.
    pub(crate) description: String,
    /// The pub date.
    pub(crate) pub_date: String,
    /// The guid.
    pub(crate) guid: String,
    /// The enclosure.
    pub(crate) enclosure: Option<RssEnclosure>,
}

/// Represents an RSS enclosure element for attaching media files to feed items.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssEnclosure {
    /// The url.
    pub(crate) url: String,
    /// The length.
    #[get(type(copy))]
    pub(crate) length: u64,
    pub(crate) r#type: String,
}
