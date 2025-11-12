use super::*;

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct RssChannel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub language: String,
    pub items: Vec<RssItem>,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub guid: String,
    pub enclosure: Option<RssEnclosure>,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct RssEnclosure {
    pub url: String,
    pub length: u64,
    pub r#type: String,
}
