use super::*;

impl TrackingService {
    pub async fn save_tracking_record(ctx: &Context, request: &RequestBody) -> Result<(), String> {
        let socket_addr: String = ctx.get_socket_addr_string().await;
        let timestamp: i64 = Utc::now().timestamp_millis();
        let headers: RequestHeaders = ctx.get_request_headers().await;
        let body_str: String = String::from_utf8_lossy(request).to_string();
        let record: TrackingRecord = TrackingRecord {
            socket_addr,
            headers,
            body: body_str,
            timestamp,
        };
        TrackingMapper::insert(record)
            .await
            .map_err(|error| format!("Failed to insert tracking record: {error}"))
    }

    pub async fn query_tracking_records(
        request: TrackingQueryRequest,
    ) -> Result<TrackingQueryResponse, String> {
        let page: i64 = request.page.unwrap_or(1).max(1);
        let page_size: i64 = request.page_size.unwrap_or(20).clamp(1, 100);
        if let Some(start) = request.start_time {
            if let Some(end) = request.end_time {
                if start > end {
                    return Err("start_time must be less than or equal to end_time".to_string());
                }
            }
        }
        let (models, total): (Vec<Model>, i64) = if request.header_key.is_some() {
            let header_key: String = request.header_key.unwrap();
            TrackingMapper::query_by_header(header_key, request.header_value, page, page_size)
                .await
                .map_err(|error| format!("Failed to query by header: {error}"))?
        } else if request.body_content.is_some() {
            let content: String = request.body_content.unwrap();
            TrackingMapper::query_by_body_content(content, page, page_size)
                .await
                .map_err(|error| format!("Failed to query by body content: {error}"))?
        } else {
            TrackingMapper::query(
                request.start_time,
                request.end_time,
                request.socket_addr,
                page,
                page_size,
            )
            .await
            .map_err(|error| format!("Failed to query tracking records: {error}"))?
        };
        let records: Vec<TrackingRecordDTO> = models
            .into_iter()
            .map(|model| TrackingRecordDTO {
                id: model.id,
                socket_addr: model.socket_addr,
                headers: serde_json::from_str(&model.headers).unwrap_or_default(),
                body: model.body,
                timestamp: model.timestamp,
                created_at: model
                    .created_at
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_default(),
            })
            .collect();
        Ok(TrackingQueryResponse {
            total,
            page,
            page_size,
            records,
        })
    }
}
