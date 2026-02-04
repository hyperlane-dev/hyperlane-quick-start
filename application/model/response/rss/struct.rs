use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssChannel {
    #[get(pub(crate))]
    pub(crate) title: String,
    #[get(pub(crate))]
    pub(crate) link: String,
    #[get(pub(crate))]
    pub(crate) description: String,
    #[get(pub(crate))]
    pub(crate) language: String,
    #[get(pub(crate))]
    pub(crate) items: Vec<RssItem>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssItem {
    #[get(pub(crate))]
    pub(crate) title: String,
    #[get(pub(crate))]
    pub(crate) link: String,
    #[get(pub(crate))]
    pub(crate) description: String,
    #[get(pub(crate))]
    pub(crate) pub_date: String,
    #[get(pub(crate))]
    pub(crate) guid: String,
    #[get(pub(crate))]
    pub(crate) enclosure: Option<RssEnclosure>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RssEnclosure {
    #[get(pub(crate))]
    pub(crate) url: String,
    #[get(type(copy), pub(crate))]
    pub(crate) length: u64,
    #[get(pub(crate))]
    pub(crate) r#type: String,
}
