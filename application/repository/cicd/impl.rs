use super::*;

/// Database access methods for `PipelineRepository` using MySQL.
impl PipelineRepository {
    /// Creates a new pipeline record with the given name, description, and configuration content.
    ///
    /// # Arguments
    ///
    /// - `String`: The pipeline name.
    /// - `Option<String>`: An optional description of the pipeline.
    /// - `Option<String>`: An optional YAML configuration content.
    ///
    /// # Returns
    ///
    /// - `Result<i32, String>`: The newly created pipeline identifier, or an error message.
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

    /// Finds a pipeline by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The pipeline identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<CicdPipelineModel>, String>`: The pipeline model if found, or `None`.
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

    /// Retrieves all pipelines ordered by creation date descending.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<CicdPipelineModel>, String>`: The list of all pipeline models.
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

/// Database access methods for `RunRepository` using MySQL.
impl RunRepository {
    /// Creates a new run record for the given pipeline.
    ///
    /// # Arguments
    ///
    /// - `i32`: The pipeline identifier.
    /// - `i32`: The sequential run number.
    /// - `Option<String>`: The user who triggered the run.
    /// - `Option<String>`: The commit hash associated with the run.
    /// - `Option<String>`: The commit message associated with the run.
    ///
    /// # Returns
    ///
    /// - `Result<CicdRunModel, String>`: The created run model, or an error message.
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

    /// Finds a run by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The run identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<CicdRunModel>, String>`: The run model if found, or `None`.
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

    /// Finds all runs for the given pipeline ordered by creation date descending.
    ///
    /// # Arguments
    ///
    /// - `i32`: The pipeline identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<CicdRunModel>, String>`: The list of run models for the pipeline.
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

    /// Queries runs with cursor-based pagination, filtering by pipeline and status.
    ///
    /// # Arguments
    ///
    /// - `Option<i32>`: Optional pipeline identifier filter.
    /// - `Option<String>`: Optional status filter string.
    /// - `Option<i32>`: Optional last ID for cursor-based pagination.
    /// - `u64`: The page size.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<CicdRunModel>, i32, bool), String>`: The runs, total count, and has-more flag.
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

    /// Retrieves the next sequential run number for the given pipeline.
    ///
    /// # Arguments
    ///
    /// - `i32`: The pipeline identifier.
    ///
    /// # Returns
    ///
    /// - `Result<i32, String>`: The next run number (max existing + 1).
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

    /// Marks a run as started by setting its status to Running and recording the start timestamp.
    ///
    /// # Arguments
    ///
    /// - `i32`: The run identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message.
    #[instrument_trace]
    pub async fn start(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Local::now().naive_local();
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

    /// Marks a run as completed with the given status, recording the completion timestamp and duration.
    ///
    /// # Arguments
    ///
    /// - `i32`: The run identifier.
    /// - `CicdStatus`: The final status (Success or Failure).
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message.
    #[instrument_trace]
    pub async fn complete(id: i32, status: CicdStatus) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let run: Option<CicdRunModel> = CicdRunEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        if let Some(run_model) = run {
            let now: NaiveDateTime = Local::now().naive_local();
            let started_at: NaiveDateTime = run_model.try_get_started_at().unwrap_or_else(|| now);
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

    /// Updates only the status of a run by its identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The run identifier.
    /// - `CicdStatus`: The new status to set.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message.
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

    /// Finds all runs with the specified status.
    ///
    /// # Arguments
    ///
    /// - `CicdStatus`: The status to filter by.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<CicdRunModel>, String>`: The list of runs matching the status.
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

/// Database access methods for `JobRepository` using MySQL.
impl JobRepository {
    /// Creates a new job associated with the given run.
    ///
    /// # Arguments
    ///
    /// - `i32`: The run identifier to associate the job with.
    /// - `String`: The name of the job.
    ///
    /// # Returns
    ///
    /// - `Result<CicdJobModel, String>`: The created job model, or an error message.
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

    /// Finds a job by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The job identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<CicdJobModel>, String>`: The job model if found, or `None`.
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

    /// Finds all jobs belonging to the specified run, ordered by creation time.
    ///
    /// # Arguments
    ///
    /// - `i32`: The run identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<CicdJobModel>, String>`: The list of jobs for the run.
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

    /// Updates the status of a job, setting timestamps and runner info based on the status.
    ///
    /// # Arguments
    ///
    /// - `i32`: The job identifier.
    /// - `CicdStatus`: The new status to set.
    /// - `Option<String>`: The optional runner identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message.
    #[instrument_trace]
    pub async fn update_status(
        id: i32,
        status: CicdStatus,
        runner: Option<String>,
    ) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Local::now().naive_local();
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
                    job_model.try_get_started_at().unwrap_or_else(|| now);
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

    /// Finds all jobs for a given run that match the specified status.
    ///
    /// # Arguments
    ///
    /// - `i32`: The run identifier.
    /// - `CicdStatus`: The status to filter by.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<CicdJobModel>, String>`: The list of matching jobs.
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

/// Database access methods for `StepRepository` using MySQL.
impl StepRepository {
    /// Creates a new step associated with the given job.
    ///
    /// # Arguments
    ///
    /// - `i32`: The job identifier to associate the step with.
    /// - `String`: The name of the step.
    /// - `Option<String>`: The optional shell command to execute.
    ///
    /// # Returns
    ///
    /// - `Result<CicdStepModel, String>`: The created step model, or an error message.
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

    /// Finds a step by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The step identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<CicdStepModel>, String>`: The step model if found, or `None`.
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

    /// Finds all steps belonging to the specified job, ordered by creation time.
    ///
    /// # Arguments
    ///
    /// - `i32`: The job identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<CicdStepModel>, String>`: The list of steps for the job.
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

    /// Updates the status of a step, setting timestamps and output based on the status.
    ///
    /// # Arguments
    ///
    /// - `i32`: The step identifier.
    /// - `CicdStatus`: The new status to set.
    /// - `Option<String>`: The optional step output content.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message.
    #[instrument_trace]
    pub async fn update_status(
        id: i32,
        status: CicdStatus,
        output: Option<String>,
    ) -> Result<(), String> {
        let db: DatabaseConnection =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Local::now().naive_local();
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
                let started_at: NaiveDateTime =
                    step_model.try_get_started_at().unwrap_or_else(|| now);
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

    /// Finds all steps for a given job that match the specified status.
    ///
    /// # Arguments
    ///
    /// - `i32`: The job identifier.
    /// - `CicdStatus`: The status to filter by.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<CicdStepModel>, String>`: The list of matching steps.
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
