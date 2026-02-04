use super::*;

#[skip_serializing_none]
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct TrackingQueryRequest {
    #[get(pub)]
    pub(super) start_time: Option<i64>,
    #[get(pub)]
    pub(super) end_time: Option<i64>,
    #[get(pub)]
    pub(super) socket_addr: Option<String>,
    #[get(pub)]
    pub(super) header_key: Option<String>,
    #[get(pub)]
    pub(super) header_value: Option<String>,
    #[get(pub)]
    pub(super) body_content: Option<String>,
    #[get(pub)]
    pub(super) page: Option<i64>,
    #[get(pub)]
    pub(super) page_size: Option<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TrackingQueryResponse {
    #[get(type(copy), pub)]
    pub(super) total: i64,
    #[get(type(copy), pub)]
    pub(super) page: i64,
    #[get(type(copy), pub)]
    pub(super) page_size: i64,
    #[get(pub)]
    pub(super) records: Vec<TrackingRecordDTO>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TrackingRecordDTO {
    #[get(type(copy), pub)]
    pub(super) id: i64,
    #[get(pub)]
    pub(super) socket_addr: String,
    #[get(pub)]
    pub(super) headers: serde_json::Value,
    #[get(pub)]
    pub(super) body: String,
    #[get(type(copy), pub)]
    pub(super) timestamp: i64,
    #[get(pub)]
    pub(super) created_at: String,
}
