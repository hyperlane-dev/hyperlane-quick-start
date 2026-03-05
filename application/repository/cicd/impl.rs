use super::*;

impl PipelineRepository {
    #[instrument_trace]
    pub async fn create(
        name: String,
        description: Option<String>,
        config_content: Option<String>,
    ) -> Result<i32, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let active_model: CicdPipelineActiveModel =
            CicdPipelineActiveModel::new(name, description, config_content);
        let result: CicdPipelineModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.get_id())
    }

    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<CicdPipelineModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<CicdPipelineModel> = CicdPipelineEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_all() -> Result<Vec<CicdPipelineModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdPipelineModel> = CicdPipelineEntity::find()
            .order_by_desc(CicdPipelineColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models)
    }
}

impl RunRepository {
    #[instrument_trace]
    pub async fn create(
        pipeline_id: i32,
        run_number: i32,
        triggered_by: Option<String>,
        commit_hash: Option<String>,
        commit_message: Option<String>,
    ) -> Result<CicdRunModel, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let active_model: CicdRunActiveModel = CicdRunActiveModel::new(
            pipeline_id,
            run_number,
            triggered_by,
            commit_hash,
            commit_message,
        );
        let result: CicdRunModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<CicdRunModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<CicdRunModel> = CicdRunEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_pipeline(pipeline_id: i32) -> Result<Vec<CicdRunModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdRunModel> = CicdRunEntity::find()
            .filter(CicdRunColumn::PipelineId.eq(pipeline_id))
            .order_by_desc(CicdRunColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models)
    }

    #[instrument_trace]
    pub async fn query_with_pagination(
        pipeline_id: Option<i32>,
        status: Option<String>,
        last_id: Option<i32>,
        page_size: u64,
    ) -> Result<(Vec<CicdRunModel>, i32, bool), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let mut query: Select<CicdRunEntity> = CicdRunEntity::find();
        if let Some(pid) = pipeline_id {
            query = query.filter(CicdRunColumn::PipelineId.eq(pid));
        }
        if let Some(s) = status {
            query = query.filter(CicdRunColumn::Status.eq(s));
        }
        if let Some(lid) = last_id {
            query = query.filter(CicdRunColumn::Id.lt(lid));
        }
        let total: i32 = CicdRunEntity::find()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i32;
        let models: Vec<CicdRunModel> = query
            .order_by_desc(CicdRunColumn::Id)
            .limit(page_size)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let has_more: bool = models.len() > (page_size - 1) as usize;
        let result: Vec<CicdRunModel> = if has_more {
            models.into_iter().take((page_size - 1) as usize).collect()
        } else {
            models
        };
        Ok((result, total, has_more))
    }

    #[instrument_trace]
    pub async fn get_next_run_number(pipeline_id: i32) -> Result<i32, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<(Option<i32>,)> = CicdRunEntity::find()
            .filter(CicdRunColumn::PipelineId.eq(pipeline_id))
            .select_only()
            .column_as(CicdRunColumn::RunNumber.max(), "max_number")
            .into_tuple()
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let max_number: i32 = result.and_then(|(inner,)| inner).unwrap_or(0);
        Ok(max_number + 1)
    }

    #[instrument_trace]
    pub async fn start(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        CicdRunEntity::update_many()
            .filter(CicdRunColumn::Id.eq(id))
            .col_expr(
                CicdRunColumn::Status,
                Expr::value(CicdStatus::Running.to_string()),
            )
            .col_expr(CicdRunColumn::StartedAt, Expr::value(now))
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn complete(id: i32, status: CicdStatus) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let run: Option<CicdRunModel> = CicdRunEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        if let Some(run_model) = run {
            let now: NaiveDateTime = Utc::now().naive_utc();
            let started_at: NaiveDateTime = run_model.try_get_started_at().unwrap_or(now);
            let duration_ms: i32 =
                (now.and_utc().timestamp_millis() - started_at.and_utc().timestamp_millis()) as i32;
            CicdRunEntity::update_many()
                .filter(CicdRunColumn::Id.eq(id))
                .col_expr(CicdRunColumn::Status, Expr::value(status.to_string()))
                .col_expr(CicdRunColumn::CompletedAt, Expr::value(now))
                .col_expr(CicdRunColumn::DurationMs, Expr::value(duration_ms))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn update_status(id: i32, status: CicdStatus) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        CicdRunEntity::update_many()
            .filter(CicdRunColumn::Id.eq(id))
            .col_expr(CicdRunColumn::Status, Expr::value(status.to_string()))
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn find_by_status(status: CicdStatus) -> Result<Vec<CicdRunModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdRunModel> = CicdRunEntity::find()
            .filter(CicdRunColumn::Status.eq(status.to_string()))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models)
    }
}

impl JobRepository {
    #[instrument_trace]
    pub async fn create(run_id: i32, name: String) -> Result<CicdJobModel, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let active_model: CicdJobActiveModel = CicdJobActiveModel::new(run_id, name);
        let result: CicdJobModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<CicdJobModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<CicdJobModel> = CicdJobEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_run(run_id: i32) -> Result<Vec<CicdJobModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdJobModel> = CicdJobEntity::find()
            .filter(CicdJobColumn::RunId.eq(run_id))
            .order_by_asc(CicdJobColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models)
    }

    #[instrument_trace]
    pub async fn update_status(
        id: i32,
        status: CicdStatus,
        runner: Option<String>,
    ) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        if status == CicdStatus::Running {
            CicdJobEntity::update_many()
                .filter(CicdJobColumn::Id.eq(id))
                .col_expr(CicdJobColumn::Status, Expr::value(status.to_string()))
                .col_expr(CicdJobColumn::Runner, Expr::value(runner))
                .col_expr(CicdJobColumn::StartedAt, Expr::value(now))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        } else if status.is_terminal() {
            let job: Option<CicdJobModel> = CicdJobEntity::find_by_id(id)
                .one(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            if let Some(job_model) = job {
                let started_at: NaiveDateTime =
                    job_model.try_get_started_at().map(|s| s).unwrap_or(now);
                let duration_ms: i32 = (now.and_utc().timestamp_millis()
                    - started_at.and_utc().timestamp_millis())
                    as i32;
                CicdJobEntity::update_many()
                    .filter(CicdJobColumn::Id.eq(id))
                    .col_expr(CicdJobColumn::Status, Expr::value(status.to_string()))
                    .col_expr(CicdJobColumn::CompletedAt, Expr::value(now))
                    .col_expr(CicdJobColumn::DurationMs, Expr::value(duration_ms))
                    .exec(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
            }
        } else {
            CicdJobEntity::update_many()
                .filter(CicdJobColumn::Id.eq(id))
                .col_expr(CicdJobColumn::Status, Expr::value(status.to_string()))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn find_by_run_and_status(
        run_id: i32,
        status: CicdStatus,
    ) -> Result<Vec<CicdJobModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdJobModel> = CicdJobEntity::find()
            .filter(CicdJobColumn::RunId.eq(run_id))
            .filter(CicdJobColumn::Status.eq(status.to_string()))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models)
    }
}

impl StepRepository {
    #[instrument_trace]
    pub async fn create(
        job_id: i32,
        name: String,
        command: Option<String>,
    ) -> Result<CicdStepModel, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let active_model: CicdStepActiveModel = CicdStepActiveModel::new(job_id, name, command);
        let result: CicdStepModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<CicdStepModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<CicdStepModel> = CicdStepEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_job(job_id: i32) -> Result<Vec<CicdStepModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdStepModel> = CicdStepEntity::find()
            .filter(CicdStepColumn::JobId.eq(job_id))
            .order_by_asc(CicdStepColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models)
    }

    #[instrument_trace]
    pub async fn update_status(
        id: i32,
        status: CicdStatus,
        output: Option<String>,
    ) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        if status == CicdStatus::Running {
            CicdStepEntity::update_many()
                .filter(CicdStepColumn::Id.eq(id))
                .col_expr(CicdStepColumn::Status, Expr::value(status.to_string()))
                .col_expr(CicdStepColumn::StartedAt, Expr::value(now))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        } else if status.is_terminal() {
            let step: Option<CicdStepModel> = CicdStepEntity::find_by_id(id)
                .one(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            if let Some(step_model) = step {
                let started_at: NaiveDateTime = (*step_model.try_get_started_at()).unwrap_or(now);
                let duration_ms: i32 = (now.and_utc().timestamp_millis()
                    - started_at.and_utc().timestamp_millis())
                    as i32;
                CicdStepEntity::update_many()
                    .filter(CicdStepColumn::Id.eq(id))
                    .col_expr(CicdStepColumn::Status, Expr::value(status.to_string()))
                    .col_expr(CicdStepColumn::Output, Expr::value(output.clone()))
                    .col_expr(CicdStepColumn::CompletedAt, Expr::value(now))
                    .col_expr(CicdStepColumn::DurationMs, Expr::value(duration_ms))
                    .exec(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
            }
        } else {
            CicdStepEntity::update_many()
                .filter(CicdStepColumn::Id.eq(id))
                .col_expr(CicdStepColumn::Status, Expr::value(status.to_string()))
                .col_expr(CicdStepColumn::Output, Expr::value(output.clone()))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn find_by_job_and_status(
        job_id: i32,
        status: CicdStatus,
    ) -> Result<Vec<CicdStepModel>, String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdStepModel> = CicdStepEntity::find()
            .filter(CicdStepColumn::JobId.eq(job_id))
            .filter(CicdStepColumn::Status.eq(status.to_string()))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models)
    }
}
