use super::*;

impl TrackingRepository {
    #[instrument_trace]
    pub fn get_db_connection() -> &'static DatabaseConnection {
        TRACKING_DB_CONNECTION.get_or_init(|| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let db: DatabaseConnection =
                        PostgreSqlPlugin::connection_db(DEFAULT_POSTGRESQL_INSTANCE_NAME, None)
                            .await
                            .expect("Failed to connect to PostgreSQL database");
                    db
                })
            })
        })
    }

    #[instrument_trace]
    pub async fn insert(record: TrackingRecord) -> Result<(), DbErr> {
        let headers_json: String = serde_json::to_string(record.get_headers())
            .map_err(|error| DbErr::Custom(format!("Failed to serialize headers {error}")))?;
        spawn(async move {
            let active_model: ActiveModel = ActiveModel {
                socket_addr: ActiveValue::Set(record.get_socket_addr().clone()),
                headers: ActiveValue::Set(headers_json),
                body: ActiveValue::Set(record.get_body().clone()),
                timestamp: ActiveValue::Set(record.get_timestamp()),
                ..Default::default()
            };
            let db: &DatabaseConnection = Self::get_db_connection();
            Entity::insert(active_model).exec(db).await.unwrap();
        });
        Ok(())
    }

    #[instrument_trace]
    pub async fn query(query: &TrackingQuery) -> Result<(Vec<Model>, i64), DbErr> {
        let db: &DatabaseConnection = Self::get_db_connection();
        let mut select: Select<Entity> = Entity::find();
        if let Some(start) = query.try_get_start_time() {
            select = select.filter(Column::Timestamp.gte(start));
        }
        if let Some(end) = query.try_get_end_time() {
            select = select.filter(Column::Timestamp.lte(end));
        }
        if let Some(addr) = query.try_get_socket_addr() {
            select = select.filter(Column::SocketAddr.contains(addr));
        }
        if let Some(cache) = query.try_get_cache_id() {
            select = select.filter(Column::Id.lte(cache));
        }
        let total: i64 = select.clone().count(db).await? as i64;
        let records: Vec<Model> = select
            .order_by_desc(Column::CreatedAt)
            .offset(((query.get_page() - 1) * query.get_page_size()) as u64)
            .limit(query.get_page_size() as u64)
            .all(db)
            .await?;
        Ok((records, total))
    }

    #[instrument_trace]
    pub async fn query_by_header(query: &TrackingHeaderQuery) -> Result<(Vec<Model>, i64), DbErr> {
        let db: &DatabaseConnection = Self::get_db_connection();
        let mut select: Select<Entity> = Entity::find();
        if let Some(start) = query.try_get_start_time() {
            select = select.filter(Column::Timestamp.gte(start));
        }
        if let Some(end) = query.try_get_end_time() {
            select = select.filter(Column::Timestamp.lte(end));
        }
        if let Some(addr) = query.try_get_socket_addr() {
            select = select.filter(Column::SocketAddr.contains(addr));
        }
        if let Some(cache) = query.try_get_cache_id() {
            select = select.filter(Column::Id.lte(cache));
        }
        let all_records: Vec<Model> = select.order_by_desc(Column::CreatedAt).all(db).await?;
        let header_key: String = query
            .try_get_header_key()
            .as_ref()
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        let header_value: &Option<String> = query.try_get_header_value();
        let filtered_records: Vec<Model> = all_records
            .into_iter()
            .filter(|record| {
                if let Ok(headers) =
                    serde_json::from_str::<HashMap<String, Vec<String>>>(record.get_headers())
                {
                    for (key, values) in headers.iter() {
                        if key.to_lowercase().contains(&header_key) {
                            if let Some(expected_value) = header_value {
                                let expected_lower: String = expected_value.to_lowercase();
                                for value in values {
                                    if value.to_lowercase().contains(&expected_lower) {
                                        return true;
                                    }
                                }
                            } else {
                                return true;
                            }
                        }
                    }
                } else if let Ok(headers) =
                    serde_json::from_str::<HashMap<String, String>>(record.get_headers())
                {
                    for (key, value) in headers.iter() {
                        if key.to_lowercase().contains(&header_key) {
                            if let Some(expected_value) = header_value {
                                if value
                                    .to_lowercase()
                                    .contains(&expected_value.to_lowercase())
                                {
                                    return true;
                                }
                            } else {
                                return true;
                            }
                        }
                    }
                }
                false
            })
            .collect();
        let total: i64 = filtered_records.len() as i64;
        let page: i64 = query.get_page();
        let page_size: i64 = query.get_page_size();
        let start: usize = ((page - 1) * page_size) as usize;
        let end: usize = (start + page_size as usize).min(filtered_records.len());
        let paginated_records: Vec<Model> = if start < filtered_records.len() {
            filtered_records[start..end].to_vec()
        } else {
            vec![]
        };
        Ok((paginated_records, total))
    }

    #[instrument_trace]
    pub async fn query_by_body_content(
        query: &TrackingBodyQuery,
    ) -> Result<(Vec<Model>, i64), DbErr> {
        let db: &DatabaseConnection = Self::get_db_connection();
        let mut select: Select<Entity> = Entity::find();
        if let Some(start) = query.try_get_start_time() {
            select = select.filter(Column::Timestamp.gte(start));
        }
        if let Some(end) = query.try_get_end_time() {
            select = select.filter(Column::Timestamp.lte(end));
        }
        if let Some(addr) = query.try_get_socket_addr() {
            select = select.filter(Column::SocketAddr.contains(addr));
        }
        if let Some(cache) = query.try_get_cache_id() {
            select = select.filter(Column::Id.lte(cache));
        }
        let all_records: Vec<Model> = select.order_by_desc(Column::CreatedAt).all(db).await?;
        let content: String = query.try_get_body_content().clone().unwrap_or_default();
        let filtered_records: Vec<Model> = all_records
            .into_iter()
            .filter(|record| record.get_body().contains(&content))
            .collect();
        let total: i64 = filtered_records.len() as i64;
        let page: i64 = query.get_page();
        let page_size: i64 = query.get_page_size();
        let start: usize = ((page - 1) * page_size) as usize;
        let end: usize = (start + page_size as usize).min(filtered_records.len());
        let paginated_records: Vec<Model> = if start < filtered_records.len() {
            filtered_records[start..end].to_vec()
        } else {
            vec![]
        };
        Ok((paginated_records, total))
    }
}
