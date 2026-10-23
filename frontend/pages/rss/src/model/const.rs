/// The name of the channel element of the feed.
pub(crate) const TAG_CHANNEL: &str = "channel";

/// The name of the item element of the feed.
pub(crate) const TAG_ITEM: &str = "item";

/// The name of the title element of the feed.
pub(crate) const TAG_TITLE: &str = "title";

/// The name of the link element of the feed.
pub(crate) const TAG_LINK: &str = "link";

/// The name of the description element of the feed.
pub(crate) const TAG_DESCRIPTION: &str = "description";

/// The name of the publication date element of the feed.
pub(crate) const TAG_PUB_DATE: &str = "pubDate";

/// The name of the identity element of the feed.
pub(crate) const TAG_GUID: &str = "guid";

/// The name of the media element of the feed.
pub(crate) const TAG_ENCLOSURE: &str = "enclosure";

/// The name of the address attribute of the media element.
pub(crate) const ATTR_URL: &str = "url";

/// The name of the size attribute of the media element.
pub(crate) const ATTR_LENGTH: &str = "length";

/// The name of the content type attribute of the media element.
pub(crate) const ATTR_TYPE: &str = "type";

/// The entity of a less-than sign.
pub(crate) const ENTITY_LT: &str = "&lt;";

/// The entity of a greater-than sign.
pub(crate) const ENTITY_GT: &str = "&gt;";

/// The entity of a double quote.
pub(crate) const ENTITY_QUOT: &str = "&quot;";

/// The entity of a single quote.
pub(crate) const ENTITY_APOS: &str = "&apos;";

/// The entity of an ampersand.
pub(crate) const ENTITY_AMP: &str = "&amp;";

/// The failure reported when the feed holds no channel.
pub(crate) const MESSAGE_NO_CHANNEL: &str = "the feed holds no channel";
