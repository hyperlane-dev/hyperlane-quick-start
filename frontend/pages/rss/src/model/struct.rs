use super::*;

/// The media file attached to one item of the feed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RssEnclosure {
    /// The address of the file.
    pub url: String,
    /// The size of the file in bytes.
    pub length: String,
    /// The content type of the file.
    pub file_type: String,
}

/// One item of the feed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RssItem {
    /// The title of the item.
    pub title: String,
    /// The address the item points at.
    pub link: String,
    /// The description of the item.
    pub description: String,
    /// The moment the item was published.
    pub pub_date: String,
    /// The stable identity of the item.
    pub guid: String,
    /// The media file attached to the item.
    pub enclosure: Option<RssEnclosure>,
}

/// The channel of the feed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RssChannel {
    /// The title of the channel.
    pub title: String,
    /// The address of the channel.
    pub link: String,
    /// The description of the channel.
    pub description: String,
    /// The items of the channel.
    pub items: Vec<RssItem>,
}
