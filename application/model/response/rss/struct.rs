use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssChannel {
    pub(crate) title: String,
    pub(crate) link: String,
    pub(crate) description: String,
    pub(crate) language: String,
    pub(crate) items: Vec<RssItem>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssItem {
    pub(crate) title: String,
    pub(crate) link: String,
    pub(crate) description: String,
    pub(crate) pub_date: String,
    pub(crate) guid: String,
    pub(crate) enclosure: Option<RssEnclosure>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssEnclosure {
    pub(crate) url: String,
    #[get(type(copy))]
    pub(crate) length: u64,
    pub(crate) r#type: String,
}
