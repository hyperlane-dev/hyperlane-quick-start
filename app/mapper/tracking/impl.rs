use super::*;

impl ActiveModelBehavior for ActiveModel {}

impl TrackingMapper {
    pub async fn insert(record: TrackingRecord) -> Result<(), DbErr> {
        let headers_json: String = serde_json::to_string(&record.headers)
            .map_err(|error| DbErr::Custom(format!("Failed to serialize headers: {error}")))?;
        spawn(async move {
            let active_model: ActiveModel = ActiveModel {
                socket_addr: ActiveValue::Set(record.socket_addr),
                headers: ActiveValue::Set(headers_json),
                body: ActiveValue::Set(record.body),
                timestamp: ActiveValue::Set(record.timestamp),
                ..Default::default()
            };
            let db: &DatabaseConnection = get_tracking_db_connection();
            Entity::insert(active_model).exec(db).await.unwrap();
        });
        Ok(())
    }

    pub async fn query(
        start_time: Option<i64>,
        end_time: Option<i64>,
        socket_addr: Option<String>,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<Model>, i64), DbErr> {
        let db: &DatabaseConnection = get_tracking_db_connection();
        let mut query: Select<Entity> = Entity::find();
        if let Some(start) = start_time {
            query = query.filter(Column::Timestamp.gte(start));
        }
        if let Some(end) = end_time {
            query = query.filter(Column::Timestamp.lte(end));
        }
        if let Some(addr) = socket_addr {
            query = query.filter(Column::SocketAddr.contains(&addr));
        }
        let total: i64 = query.clone().count(db).await? as i64;
        let records: Vec<Model> = query
            .order_by_desc(Column::CreatedAt)
            .offset(((page - 1) * page_size) as u64)
            .limit(page_size as u64)
            .all(db)
            .await?;
        Ok((records, total))
    }

    pub async fn query_by_header(
        header_key: String,
        header_value: Option<String>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        socket_addr: Option<String>,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<Model>, i64), DbErr> {
        let db: &DatabaseConnection = get_tracking_db_connection();
        let mut query: Select<Entity> = Entity::find();
        if let Some(start) = start_time {
            query = query.filter(Column::Timestamp.gte(start));
        }
        if let Some(end) = end_time {
            query = query.filter(Column::Timestamp.lte(end));
        }
        if let Some(addr) = socket_addr {
            query = query.filter(Column::SocketAddr.contains(&addr));
        }
        let all_records: Vec<Model> = query.order_by_desc(Column::CreatedAt).all(db).await?;
        let header_key_lower: String = header_key.to_lowercase();
        let filtered_records: Vec<Model> = all_records
            .into_iter()
            .filter(|record| {
                if let Ok(headers) =
                    serde_json::from_str::<HashMap<String, Vec<String>>>(&record.headers)
                {
                    for (key, values) in headers.iter() {
                        if key.to_lowercase().contains(&header_key_lower) {
                            if let Some(ref expected_value) = header_value {
                                let expected_lower = expected_value.to_lowercase();
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
                    serde_json::from_str::<HashMap<String, String>>(&record.headers)
                {
                    for (key, value) in headers.iter() {
                        if key.to_lowercase().contains(&header_key_lower) {
                            if let Some(ref expected_value) = header_value {
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
        let start: usize = ((page - 1) * page_size) as usize;
        let end: usize = (start + page_size as usize).min(filtered_records.len());
        let paginated_records: Vec<Model> = if start < filtered_records.len() {
            filtered_records[start..end].to_vec()
        } else {
            vec![]
        };
        Ok((paginated_records, total))
    }

    pub async fn query_by_body_content(
        content: String,
        start_time: Option<i64>,
        end_time: Option<i64>,
        socket_addr: Option<String>,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<Model>, i64), DbErr> {
        let db: &DatabaseConnection = get_tracking_db_connection();
        let mut query: Select<Entity> = Entity::find();
        if let Some(start) = start_time {
            query = query.filter(Column::Timestamp.gte(start));
        }
        if let Some(end) = end_time {
            query = query.filter(Column::Timestamp.lte(end));
        }
        if let Some(addr) = socket_addr {
            query = query.filter(Column::SocketAddr.contains(&addr));
        }
        let all_records: Vec<Model> = query.order_by_desc(Column::CreatedAt).all(db).await?;
        let filtered_records: Vec<Model> = all_records
            .into_iter()
            .filter(|record| record.body.contains(&content))
            .collect();
        let total: i64 = filtered_records.len() as i64;
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

pub fn get_tracking_db_connection() -> &'static DatabaseConnection {
    TRACKING_DB_CONNECTION.get_or_init(|| {
        let env: &EnvConfig = get_global_env_config();
        let db_url: String = if *env.get_enable_mysql() {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                env.get_mysql_username(),
                env.get_mysql_password(),
                env.get_mysql_host(),
                env.get_mysql_port(),
                env.get_mysql_database()
            )
        } else if *env.get_enable_postgresql() {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                env.get_postgresql_username(),
                env.get_postgresql_password(),
                env.get_postgresql_host(),
                env.get_postgresql_port(),
                env.get_postgresql_database()
            )
        } else {
            panic!("No database enabled in configuration");
        };
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                Database::connect(&db_url)
                    .await
                    .expect("Failed to connect to database")
            })
        })
    })
}
