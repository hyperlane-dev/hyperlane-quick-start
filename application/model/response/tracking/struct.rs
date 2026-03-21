use super::*;

#[skip_serializing_none]
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct TrackingQueryRequest {
    #[get(type(copy))]
    pub(super) start_time: Option<i64>,
    #[get(type(copy))]
    pub(super) end_time: Option<i64>,
    pub(super) header_key: Option<String>,
    pub(super) header_value: Option<String>,
    pub(super) body_content: Option<String>,
    #[get(type(copy))]
    pub(super) page: Option<i64>,
    #[get(type(copy))]
    pub(super) page_size: Option<i64>,
    #[get(type(copy))]
    pub(super) cache_id: Option<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TrackingQueryResponse {
    #[get(type(copy))]
    pub(super) total: i64,
    #[get(type(copy))]
    pub(super) page: i64,
    #[get(type(copy))]
    pub(super) page_size: i64,
    pub(super) records: Vec<TrackingRecordDTO>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TrackingRecordDTO {
    #[get(type(copy))]
    pub(super) id: i64,
    pub(super) headers: serde_json::Value,
    pub(super) body: String,
    #[get(type(copy))]
    pub(super) timestamp: i64,
    pub(super) created_at: i64,
}
