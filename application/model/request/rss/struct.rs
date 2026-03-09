use super::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema, Data)]
pub struct RssFeedRequest {
    #[get(type(copy))]
    pub limit: Option<usize>,
    #[get(type(copy))]
    pub offset: Option<usize>,
    pub timezone: Option<Timezone>,
}
