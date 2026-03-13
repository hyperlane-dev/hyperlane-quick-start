use super::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct TrackingRepository;

#[derive(Clone, Data, Debug, Default)]
pub struct TrackingQuery {
    #[get(type(copy))]
    pub(super) start_time: Option<i64>,
    #[get(type(copy))]
    pub(super) end_time: Option<i64>,
    #[get(type(copy))]
    pub(super) page: i64,
    #[get(type(copy))]
    pub(super) page_size: i64,
    #[get(type(copy))]
    pub(super) cache_id: Option<i64>,
}

#[derive(Clone, Data, Debug, Default)]
pub struct TrackingHeaderQuery {
    pub(super) header_key: Option<String>,
    pub(super) header_value: Option<String>,
    #[get(type(copy))]
    pub(super) start_time: Option<i64>,
    #[get(type(copy))]
    pub(super) end_time: Option<i64>,
    #[get(type(copy))]
    pub(super) page: i64,
    #[get(type(copy))]
    pub(super) page_size: i64,
    #[get(type(copy))]
    pub(super) cache_id: Option<i64>,
}

#[derive(Clone, Data, Debug, Default)]
pub struct TrackingBodyQuery {
    pub(super) body_content: Option<String>,
    #[get(type(copy))]
    pub(super) start_time: Option<i64>,
    #[get(type(copy))]
    pub(super) end_time: Option<i64>,
    #[get(type(copy))]
    pub(super) page: i64,
    #[get(type(copy))]
    pub(super) page_size: i64,
    #[get(type(copy))]
    pub(super) cache_id: Option<i64>,
}
