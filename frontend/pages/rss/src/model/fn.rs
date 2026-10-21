use super::*;

/// Reads the channel of an RSS document.
///
/// The document the server answers with is a flat RSS 2.0 feed: a channel
/// that holds the items, and every item that holds the elements the reader
/// needs. The reader walks the document and collects those elements.
///
/// # Arguments
///
/// - `&str` - The document the server answered with.
///
/// # Returns
///
/// - `Result<RssChannel, String>` - The channel of the feed, or the failure that stopped the reader.
pub fn parse_feed(xml: &str) -> Result<RssChannel, String> {
    let channel: String = match element(xml, TAG_CHANNEL) {
        Some(channel) => channel,
        None => return Err(MESSAGE_NO_CHANNEL.to_string()),
    };
    let mut items: Vec<RssItem> = Vec::new();
    for block in elements(&channel, TAG_ITEM) {
        items.push(parse_item(&block));
    }
    Ok(RssChannel {
        title: text(&channel, TAG_TITLE),
        link: text(&channel, TAG_LINK),
        description: text(&channel, TAG_DESCRIPTION),
        items,
    })
}

/// Reads one item of the feed.
///
/// # Arguments
///
/// - `&str` - The content of the item element.
///
/// # Returns
///
/// - `RssItem` - The item the feed holds.
fn parse_item(block: &str) -> RssItem {
    RssItem {
        title: text(block, TAG_TITLE),
        link: text(block, TAG_LINK),
        description: text(block, TAG_DESCRIPTION),
        pub_date: text(block, TAG_PUB_DATE),
        guid: text(block, TAG_GUID),
        enclosure: enclosure(block),
    }
}

/// Reads the media file attached to an item.
///
/// # Arguments
///
/// - `&str` - The content of the item element.
///
/// # Returns
///
/// - `Option<RssEnclosure>` - The media file, absent when the item holds none.
fn enclosure(block: &str) -> Option<RssEnclosure> {
    let start: usize = block.find(&format!("<{TAG_ENCLOSURE}"))?;
    let end: usize = block[start..].find('>')? + start;
    let tag: &str = &block[start..end];
    Some(RssEnclosure {
        url: attribute(tag, ATTR_URL),
        length: attribute(tag, ATTR_LENGTH),
        file_type: attribute(tag, ATTR_TYPE),
    })
}

/// Reads the content of the first element the caller names.
///
/// # Arguments
///
/// - `&str` - The document to read.
/// - `&str` - The name of the element.
///
/// # Returns
///
/// - `Option<String>` - The content of the element, absent when the document holds none.
fn element(xml: &str, tag: &str) -> Option<String> {
    let open: String = open_tag(tag);
    let close: String = close_tag(tag);
    let start: usize = xml.find(&open)? + open.len();
    let end: usize = xml[start..].find(&close)? + start;
    Some(xml[start..end].to_string())
}

/// Reads the content of every element the caller names.
///
/// # Arguments
///
/// - `&str` - The document to read.
/// - `&str` - The name of the element.
///
/// # Returns
///
/// - `Vec<String>` - The content of the elements, oldest first.
fn elements(xml: &str, tag: &str) -> Vec<String> {
    let open: String = open_tag(tag);
    let close: String = close_tag(tag);
    let mut blocks: Vec<String> = Vec::new();
    let mut rest: &str = xml;
    while let Some(found) = rest.find(&open) {
        let start: usize = found + open.len();
        let Some(end) = rest[start..].find(&close) else {
            break;
        };
        blocks.push(rest[start..start + end].to_string());
        rest = &rest[start + end + close.len()..];
    }
    blocks
}

/// Builds the opening tag of an element.
///
/// # Arguments
///
/// - `&str` - The name of the element.
///
/// # Returns
///
/// - `String` - The opening tag of the element.
fn open_tag(tag: &str) -> String {
    format!("<{tag}>")
}

/// Builds the closing tag of an element.
///
/// # Arguments
///
/// - `&str` - The name of the element.
///
/// # Returns
///
/// - `String` - The closing tag of the element.
fn close_tag(tag: &str) -> String {
    format!("</{tag}>")
}

/// Reads the text of the first element the caller names.
///
/// # Arguments
///
/// - `&str` - The document to read.
/// - `&str` - The name of the element.
///
/// # Returns
///
/// - `String` - The text of the element, empty when the document holds none.
fn text(xml: &str, tag: &str) -> String {
    match element(xml, tag) {
        Some(content) => unescape(&content).trim().to_string(),
        None => String::new(),
    }
}

/// Reads one attribute of a tag.
///
/// # Arguments
///
/// - `&str` - The tag to read.
/// - `&str` - The name of the attribute.
///
/// # Returns
///
/// - `String` - The value of the attribute, empty when the tag holds none.
fn attribute(tag: &str, name: &str) -> String {
    let key: String = format!("{name}=\"");
    let Some(found) = tag.find(&key) else {
        return String::new();
    };
    let start: usize = found + key.len();
    let Some(end) = tag[start..].find('"') else {
        return String::new();
    };
    unescape(&tag[start..start + end])
}

/// Reads the entities of a text of the feed.
///
/// # Arguments
///
/// - `&str` - The text to read.
///
/// # Returns
///
/// - `String` - The text the entities stand for.
fn unescape(text: &str) -> String {
    text.replace(ENTITY_LT, "<")
        .replace(ENTITY_GT, ">")
        .replace(ENTITY_QUOT, "\"")
        .replace(ENTITY_APOS, "'")
        .replace(ENTITY_AMP, "&")
}
