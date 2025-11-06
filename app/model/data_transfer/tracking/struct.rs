use super::*;

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TrackingQueryRequest {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub socket_addr: Option<String>,
    pub header_key: Option<String>,
    pub header_value: Option<String>,
    pub body_content: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct TrackingQueryResponse {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub records: Vec<TrackingRecordDTO>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct TrackingRecordDTO {
    pub id: i64,
    pub socket_addr: String,
    pub headers: Value,
    pub body: String,
    pub timestamp: i64,
    pub created_at: String,
}
