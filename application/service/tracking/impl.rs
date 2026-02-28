use super::*;

impl TrackingService {
    #[instrument_trace]
    pub async fn save_tracking_record(
        ctx: &mut Context,
        request: &RequestBody,
    ) -> Result<(), String> {
        let mut socket_addr: String = String::new();
        if let Some(stream) = ctx.try_get_stream().as_ref() {
            socket_addr = stream
                .read()
                .await
                .peer_addr()
                .map(|data| data.to_string())
                .unwrap_or_default();
        }
        let timestamp: i64 = Utc::now().timestamp_millis();
        let headers: RequestHeaders = ctx.get_request().get_headers().clone();
        let body_str: String = String::from_utf8_lossy(request).to_string();
        let mut record: TrackingRecord = TrackingRecord::default();
        record
            .set_socket_addr(socket_addr)
            .set_headers(headers)
            .set_body(body_str)
            .set_timestamp(timestamp);
        TrackingMapper::insert(record)
            .await
            .map_err(|error| format!("Failed to insert tracking record {error}"))
    }

    #[instrument_trace]
    pub async fn query_tracking_records(
        request: TrackingQueryRequest,
    ) -> Result<TrackingQueryResponse, String> {
        let page: i64 = (*request.try_get_page()).unwrap_or(1).max(1);
        let page_size: i64 = (*request.try_get_page_size()).unwrap_or(20).clamp(1, 100);
        if let Some(start) = *request.try_get_start_time() {
            if let Some(end) = *request.try_get_end_time() {
                if start > end {
                    return Err("start_time must be less than or equal to end_time".to_string());
                }
            }
        }
        let (models, total): (Vec<Model>, i64) = if request.try_get_header_key().clone().is_some() {
            let header_key: String = request.try_get_header_key().clone().unwrap();
            TrackingMapper::query_by_header(
                header_key,
                request.try_get_header_value().clone(),
                *request.try_get_start_time(),
                *request.try_get_end_time(),
                request.try_get_socket_addr().clone(),
                page,
                page_size,
            )
            .await
            .map_err(|error| format!("Failed to query by header {error}"))?
        } else if request.try_get_body_content().clone().is_some() {
            let content: String = request.try_get_body_content().clone().unwrap();
            TrackingMapper::query_by_body_content(
                content,
                *request.try_get_start_time(),
                *request.try_get_end_time(),
                request.try_get_socket_addr().clone(),
                page,
                page_size,
            )
            .await
            .map_err(|error| format!("Failed to query by body content {error}"))?
        } else {
            TrackingMapper::query(
                *request.try_get_start_time(),
                *request.try_get_end_time(),
                request.try_get_socket_addr().clone(),
                page,
                page_size,
            )
            .await
            .map_err(|error| format!("Failed to query tracking records {error}"))?
        };
        let records: Vec<TrackingRecordDTO> = models
            .into_iter()
            .map(|model| {
                let mut dto = TrackingRecordDTO::default();
                dto.set_id(model.get_id())
                    .set_socket_addr(model.get_socket_addr().clone())
                    .set_headers(serde_json::from_str(model.get_headers()).unwrap_or_default())
                    .set_body(model.get_body().clone())
                    .set_timestamp(model.get_timestamp())
                    .set_created_at(
                        model
                            .try_get_created_at()
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_default(),
                    );
                dto
            })
            .collect();
        let mut response = TrackingQueryResponse::default();
        response
            .set_total(total)
            .set_page(page)
            .set_page_size(page_size)
            .set_records(records);
        Ok(response)
    }
}
