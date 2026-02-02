use super::*;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RssChannel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub language: String,
    pub items: Vec<RssItem>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub guid: String,
    pub enclosure: Option<RssEnclosure>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RssEnclosure {
    pub url: String,
    pub length: u64,
    pub r#type: String,
}
