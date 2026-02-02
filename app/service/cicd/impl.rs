use super::*;

impl CicdService {
    #[instrument_trace]
    pub async fn create_pipeline(param: CreatePipelineParam) -> Result<i32, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let active_model: PipelineActiveModel =
            PipelineActiveModel::new(param.name, param.description, param.config_content);
        let result: mapper::cicd::pipeline::Model = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.id)
    }

    #[instrument_trace]
    pub async fn update_pipeline(param: UpdatePipelineParam) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        PipelineEntity::update_many()
            .filter(PipelineColumn::Id.eq(param.id))
            .col_expr(PipelineColumn::Name, Expr::value(param.name))
            .col_expr(PipelineColumn::Description, Expr::value(param.description))
            .col_expr(
                PipelineColumn::ConfigContent,
                Expr::value(param.config_content),
            )
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn delete_pipeline(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        PipelineEntity::update_many()
            .filter(PipelineColumn::Id.eq(id))
            .filter(PipelineColumn::DeletedAt.is_null())
            .col_expr(PipelineColumn::DeletedAt, Expr::value(now))
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn get_pipeline_by_id(id: i32) -> Result<Option<PipelineDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<mapper::cicd::pipeline::Model> = PipelineEntity::find_by_id(id)
            .filter(PipelineColumn::DeletedAt.is_null())
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.map(pipeline_to_dto))
    }

    #[instrument_trace]
    pub async fn get_all_pipelines() -> Result<Vec<PipelineDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<mapper::cicd::pipeline::Model> = PipelineEntity::find()
            .filter(PipelineColumn::DeletedAt.is_null())
            .order_by_desc(PipelineColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models.into_iter().map(pipeline_to_dto).collect())
    }

    #[instrument_trace]
    pub async fn trigger_run(param: TriggerRunParam) -> Result<i32, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;

        let pipeline = Self::get_pipeline_by_id_with_config(param.pipeline_id).await?;
        let config_content = pipeline
            .as_ref()
            .and_then(|p| p.config_content.clone())
            .ok_or_else(|| "Pipeline config content is required".to_string())?;

        let run_number: i32 = Self::get_next_run_number(param.pipeline_id).await?;
        let active_model: RunActiveModel = RunActiveModel::new(
            param.pipeline_id,
            run_number,
            param.triggered_by,
            param.commit_hash,
            param.commit_message,
        );
        let run_result = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let run_id: i32 = run_result.id;

        Self::parse_config_and_create_jobs(&db, run_id, &config_content).await?;

        let run_id_clone: i32 = run_id;
        tokio::spawn(async move {
            if let Err(error) = Self::execute_run(run_id_clone).await {
                tracing::error!("Failed to execute run {}: {}", run_id_clone, error);
            }
        });

        Ok(run_id)
    }

    #[instrument_trace]
    async fn get_pipeline_by_id_with_config(
        id: i32,
    ) -> Result<Option<mapper::cicd::pipeline::Model>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<mapper::cicd::pipeline::Model> = PipelineEntity::find_by_id(id)
            .filter(PipelineColumn::DeletedAt.is_null())
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    async fn parse_config_and_create_jobs(
        db: &DatabaseConnection,
        run_id: i32,
        config_content: &str,
    ) -> Result<(), String> {
        let config: PipelineConfig = serde_yaml::from_str(config_content)
            .map_err(|error| format!("Failed to parse config: {error}"))?;

        let pipeline_dockerfile: Option<String> = config.dockerfile;
        let pipeline_image: Option<String> = config.image;

        for (job_name, job_config) in config.jobs {
            let job_dockerfile: Option<String> = job_config
                .dockerfile
                .or_else(|| pipeline_dockerfile.clone());
            let job_image: Option<String> = job_config.image.or_else(|| pipeline_image.clone());

            let job_active_model = JobActiveModel::new(run_id, job_name);
            let job_result = job_active_model
                .insert(db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            let job_id: i32 = job_result.id;

            for step_config in job_config.steps {
                let step_dockerfile: Option<String> =
                    step_config.dockerfile.or_else(|| job_dockerfile.clone());
                let step_image: Option<String> = job_image.clone();

                let step_active_model = if step_dockerfile.is_some() || step_image.is_some() {
                    StepActiveModel::new_with_dockerfile(
                        job_id,
                        step_config.name,
                        step_dockerfile,
                        step_image,
                    )
                } else {
                    StepActiveModel::new(job_id, step_config.name, step_config.run)
                };
                step_active_model
                    .insert(db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
            }
        }

        Ok(())
    }

    #[instrument_trace]
    pub async fn execute_run(run_id: i32) -> Result<(), String> {
        Self::start_run(run_id).await?;

        let jobs = Self::get_jobs_by_run(run_id).await?;
        let mut has_error: bool = false;

        for job in jobs {
            let job_id: i32 = job.id;
            Self::update_job_status(UpdateJobStatusParam {
                job_id,
                status: CicdStatus::Running,
                runner: Some("local-runner".to_string()),
            })
            .await?;

            let steps = Self::get_steps_by_job(job_id).await?;
            let mut job_has_error: bool = false;

            for step in steps {
                let step_id: i32 = step.id;

                Self::update_step_status(UpdateStepStatusParam {
                    step_id,
                    status: CicdStatus::Running,
                    output: None,
                })
                .await?;

                let output: String = if step.dockerfile.is_some() {
                    Self::execute_dockerfile(&step).await
                } else {
                    let command: String = step.command.unwrap_or_default();
                    Self::execute_command(&command).await
                };

                let step_status: CicdStatus = if output.starts_with("Error:") {
                    job_has_error = true;
                    has_error = true;
                    CicdStatus::Failure
                } else {
                    CicdStatus::Success
                };

                Self::update_step_status(UpdateStepStatusParam {
                    step_id,
                    status: step_status,
                    output: Some(output),
                })
                .await?;

                if job_has_error {
                    break;
                }
            }

            let job_status: CicdStatus = if job_has_error {
                CicdStatus::Failure
            } else {
                CicdStatus::Success
            };

            Self::update_job_status(UpdateJobStatusParam {
                job_id,
                status: job_status,
                runner: Some("local-runner".to_string()),
            })
            .await?;

            if job_has_error {
                break;
            }
        }

        let run_status: CicdStatus = if has_error {
            CicdStatus::Failure
        } else {
            CicdStatus::Success
        };

        Self::complete_run(run_id, run_status).await?;

        Ok(())
    }

    #[instrument_trace]
    async fn execute_command(command: &str) -> String {
        if command.is_empty() {
            return "No command to execute".to_string();
        }

        let config: DockerConfig = DockerConfig::secure();
        let result: DockerResult = execute_with_config(command, &config).await;
        result.format_output()
    }

    #[instrument_trace]
    pub async fn execute_dockerfile(step: &StepDto) -> String {
        let dockerfile_content: String = match &step.dockerfile {
            Some(content) => content.clone(),
            None => return "Error: No Dockerfile content".to_string(),
        };

        let run_id: i32 = step.job_id;
        let step_id: i32 = step.id;
        let image_tag: String = format!("cicd-run-{run_id}-step-{step_id}");
        let temp_dir: std::path::PathBuf =
            std::env::temp_dir().join(format!("cicd-{run_id}-{step_id}"));

        if let Err(error) = tokio::fs::create_dir_all(&temp_dir).await {
            return format!("Error: Failed to create temp directory: {error}");
        }

        let dockerfile_path: std::path::PathBuf = temp_dir.join("Dockerfile");
        if let Err(error) = tokio::fs::write(&dockerfile_path, dockerfile_content).await {
            return format!("Error: Failed to write Dockerfile: {error}");
        }

        let build_output: std::process::Output = match tokio::process::Command::new("docker")
            .args([
                "build",
                "--progress=plain",
                "-t",
                &image_tag,
                temp_dir.to_str().unwrap_or("."),
            ])
            .output()
            .await
        {
            Ok(output) => output,
            Err(error) => {
                let _ = tokio::fs::remove_dir_all(&temp_dir).await;
                return format!("Error: Failed to build Docker image: {error}");
            }
        };

        let build_stdout: String = String::from_utf8_lossy(&build_output.stdout).to_string();
        let build_stderr: String = String::from_utf8_lossy(&build_output.stderr).to_string();

        if !build_output.status.success() {
            let _ = tokio::fs::remove_dir_all(&temp_dir).await;
            let _ = tokio::process::Command::new("docker")
                .args(["rmi", &image_tag])
                .output()
                .await;
            return format!(
                "Error: Docker build failed\nStdout: {}\nStderr: {}",
                build_stdout.trim(),
                build_stderr.trim()
            );
        }

        let run_args: Vec<String> = vec![
            "run".to_string(),
            "--rm".to_string(),
            "--network=none".to_string(),
            "--cpus=1".to_string(),
            "--memory=512m".to_string(),
            "--pids-limit=100".to_string(),
            image_tag.clone(),
        ];

        let run_output: std::process::Output = match tokio::process::Command::new("docker")
            .args(&run_args)
            .output()
            .await
        {
            Ok(output) => output,
            Err(error) => {
                let _ = tokio::fs::remove_dir_all(&temp_dir).await;
                let _ = tokio::process::Command::new("docker")
                    .args(["rmi", &image_tag])
                    .output()
                    .await;
                return format!("Error: Failed to run Docker container: {error}");
            }
        };

        let run_stdout: String = String::from_utf8_lossy(&run_output.stdout).to_string();
        let run_stderr: String = String::from_utf8_lossy(&run_output.stderr).to_string();

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
        let _ = tokio::process::Command::new("docker")
            .args(["rmi", &image_tag])
            .output()
            .await;

        let mut result_parts: Vec<String> = Vec::new();

        let build_log: String = format!("{}{}", build_stdout.trim(), build_stderr.trim());
        if !build_log.is_empty() {
            result_parts.push(format!("[Build Log]\n{}", build_log.trim()));
        }

        if run_output.status.success() {
            let run_log: String = format!("{}{}", run_stdout.trim(), run_stderr.trim());
            if !run_log.is_empty() {
                result_parts.push(format!("[Run Output]\n{}", run_log.trim()));
            }
        } else {
            result_parts.push(format!(
                "[Run Failed]\nExit code: {}\nStdout: {}\nStderr: {}",
                run_output.status.code().unwrap_or(-1),
                run_stdout.trim(),
                run_stderr.trim()
            ));
        }

        if result_parts.is_empty() {
            "Dockerfile executed successfully (no output)".to_string()
        } else {
            result_parts.join("\n\n")
        }
    }

    #[instrument_trace]
    pub async fn get_next_run_number(pipeline_id: i32) -> Result<i32, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<(Option<i32>,)> = RunEntity::find()
            .filter(RunColumn::PipelineId.eq(pipeline_id))
            .select_only()
            .column_as(RunColumn::RunNumber.max(), "max_number")
            .into_tuple()
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let max_number: i32 = result.and_then(|(inner,)| inner).unwrap_or(0);
        Ok(max_number + 1)
    }

    #[instrument_trace]
    pub async fn get_run_by_id(id: i32) -> Result<Option<RunDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<mapper::cicd::run::Model> = RunEntity::find_by_id(id)
            .filter(RunColumn::DeletedAt.is_null())
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.map(run_to_dto))
    }

    #[instrument_trace]
    pub async fn get_runs_by_pipeline(pipeline_id: i32) -> Result<Vec<RunDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<mapper::cicd::run::Model> = RunEntity::find()
            .filter(RunColumn::PipelineId.eq(pipeline_id))
            .filter(RunColumn::DeletedAt.is_null())
            .order_by_desc(RunColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models.into_iter().map(run_to_dto).collect())
    }

    #[instrument_trace]
    pub async fn query_runs(param: QueryRunsParam) -> Result<PaginatedRunsDto, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let page: i32 = param.page.unwrap_or(1);
        let page_size: i32 = param.page_size.unwrap_or(10);
        let mut query = RunEntity::find().filter(RunColumn::DeletedAt.is_null());
        if let Some(pipeline_id) = param.pipeline_id {
            query = query.filter(RunColumn::PipelineId.eq(pipeline_id));
        }
        if let Some(status) = param.status {
            query = query.filter(RunColumn::Status.eq(status.to_string()));
        }
        let total: i32 = query
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i32;
        let models: Vec<mapper::cicd::run::Model> = query
            .order_by_desc(RunColumn::CreatedAt)
            .limit(page_size as u64)
            .offset(((page - 1) * page_size) as u64)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(PaginatedRunsDto {
            total,
            page,
            page_size,
            runs: models.into_iter().map(run_to_dto).collect(),
        })
    }

    #[instrument_trace]
    pub async fn update_run_status(id: i32, status: CicdStatus) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        RunEntity::update_many()
            .filter(RunColumn::Id.eq(id))
            .col_expr(RunColumn::Status, Expr::value(status.to_string()))
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn start_run(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        RunEntity::update_many()
            .filter(RunColumn::Id.eq(id))
            .col_expr(
                RunColumn::Status,
                Expr::value(CicdStatus::Running.to_string()),
            )
            .col_expr(RunColumn::StartedAt, Expr::value(now))
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn complete_run(id: i32, status: CicdStatus) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let run = RunEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        if let Some(run_model) = run {
            let now: NaiveDateTime = Utc::now().naive_utc();
            let started_at: NaiveDateTime = run_model.started_at.unwrap_or(now);
            let duration_ms: i32 =
                (now.and_utc().timestamp_millis() - started_at.and_utc().timestamp_millis()) as i32;
            RunEntity::update_many()
                .filter(RunColumn::Id.eq(id))
                .col_expr(RunColumn::Status, Expr::value(status.to_string()))
                .col_expr(RunColumn::CompletedAt, Expr::value(now))
                .col_expr(RunColumn::DurationMs, Expr::value(duration_ms))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn create_job(run_id: i32, name: String) -> Result<i32, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let active_model: JobActiveModel = JobActiveModel::new(run_id, name);
        let result: mapper::cicd::job::Model = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.id)
    }

    #[instrument_trace]
    pub async fn get_jobs_by_run(run_id: i32) -> Result<Vec<JobDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<mapper::cicd::job::Model> = JobEntity::find()
            .filter(JobColumn::RunId.eq(run_id))
            .order_by_asc(JobColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models.into_iter().map(job_to_dto).collect())
    }

    #[instrument_trace]
    pub async fn update_job_status(param: UpdateJobStatusParam) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        if param.status == CicdStatus::Running {
            JobEntity::update_many()
                .filter(JobColumn::Id.eq(param.job_id))
                .col_expr(JobColumn::Status, Expr::value(param.status.to_string()))
                .col_expr(JobColumn::Runner, Expr::value(param.runner))
                .col_expr(JobColumn::StartedAt, Expr::value(now))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        } else if param.status.is_terminal() {
            let job = JobEntity::find_by_id(param.job_id)
                .one(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            if let Some(job_model) = job {
                let started_at: NaiveDateTime = job_model.started_at.unwrap_or(now);
                let duration_ms: i32 = (now.and_utc().timestamp_millis()
                    - started_at.and_utc().timestamp_millis())
                    as i32;
                JobEntity::update_many()
                    .filter(JobColumn::Id.eq(param.job_id))
                    .col_expr(JobColumn::Status, Expr::value(param.status.to_string()))
                    .col_expr(JobColumn::CompletedAt, Expr::value(now))
                    .col_expr(JobColumn::DurationMs, Expr::value(duration_ms))
                    .exec(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
            }
        } else {
            JobEntity::update_many()
                .filter(JobColumn::Id.eq(param.job_id))
                .col_expr(JobColumn::Status, Expr::value(param.status.to_string()))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn create_step(
        job_id: i32,
        name: String,
        command: Option<String>,
    ) -> Result<i32, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let active_model: StepActiveModel = StepActiveModel::new(job_id, name, command);
        let result: mapper::cicd::step::Model = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.id)
    }

    #[instrument_trace]
    pub async fn get_steps_by_job(job_id: i32) -> Result<Vec<StepDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<mapper::cicd::step::Model> = StepEntity::find()
            .filter(StepColumn::JobId.eq(job_id))
            .order_by_asc(StepColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models.into_iter().map(step_to_dto).collect())
    }

    #[instrument_trace]
    pub async fn update_step_status(param: UpdateStepStatusParam) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        if param.status == CicdStatus::Running {
            StepEntity::update_many()
                .filter(StepColumn::Id.eq(param.step_id))
                .col_expr(StepColumn::Status, Expr::value(param.status.to_string()))
                .col_expr(StepColumn::StartedAt, Expr::value(now))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        } else if param.status.is_terminal() {
            let step = StepEntity::find_by_id(param.step_id)
                .one(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            if let Some(step_model) = step {
                let started_at: NaiveDateTime = step_model.started_at.unwrap_or(now);
                let duration_ms: i32 = (now.and_utc().timestamp_millis()
                    - started_at.and_utc().timestamp_millis())
                    as i32;
                StepEntity::update_many()
                    .filter(StepColumn::Id.eq(param.step_id))
                    .col_expr(StepColumn::Status, Expr::value(param.status.to_string()))
                    .col_expr(StepColumn::Output, Expr::value(param.output))
                    .col_expr(StepColumn::CompletedAt, Expr::value(now))
                    .col_expr(StepColumn::DurationMs, Expr::value(duration_ms))
                    .exec(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
            }
        } else {
            StepEntity::update_many()
                .filter(StepColumn::Id.eq(param.step_id))
                .col_expr(StepColumn::Status, Expr::value(param.status.to_string()))
                .col_expr(StepColumn::Output, Expr::value(param.output))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn get_run_detail(run_id: i32) -> Result<Option<RunDetailDto>, String> {
        let run = Self::get_run_by_id(run_id).await?;
        if let Some(run_dto) = run {
            let jobs = Self::get_jobs_by_run(run_id).await?;
            let mut jobs_with_steps: Vec<JobWithStepsDto> = Vec::new();
            for job in jobs {
                let steps = Self::get_steps_by_job(job.id).await?;
                jobs_with_steps.push(JobWithStepsDto { job, steps });
            }
            Ok(Some(RunDetailDto {
                run: run_dto,
                jobs: jobs_with_steps,
            }))
        } else {
            Ok(None)
        }
    }
}

#[instrument_trace]
fn pipeline_to_dto(model: mapper::cicd::pipeline::Model) -> PipelineDto {
    PipelineDto {
        id: model.id,
        name: model.name,
        description: model.description,
        config_content: model.config_content,
        created_at: model.created_at.map(|dt| dt.to_string()),
        updated_at: model.updated_at.map(|dt| dt.to_string()),
    }
}

#[instrument_trace]
fn run_to_dto(model: mapper::cicd::run::Model) -> RunDto {
    let status: CicdStatus = model.status.parse().unwrap_or_default();
    RunDto {
        id: model.id,
        pipeline_id: model.pipeline_id,
        pipeline_name: None,
        run_number: model.run_number,
        status,
        triggered_by: model.triggered_by,
        commit_hash: model.commit_hash,
        commit_message: model.commit_message,
        started_at: model.started_at.map(|dt| dt.to_string()),
        completed_at: model.completed_at.map(|dt| dt.to_string()),
        duration_ms: model.duration_ms,
        created_at: model.created_at.map(|dt| dt.to_string()),
    }
}

#[instrument_trace]
fn job_to_dto(model: mapper::cicd::job::Model) -> JobDto {
    let status: CicdStatus = model.status.parse().unwrap_or_default();
    JobDto {
        id: model.id,
        run_id: model.run_id,
        name: model.name,
        status,
        runner: model.runner,
        started_at: model.started_at.map(|dt| dt.to_string()),
        completed_at: model.completed_at.map(|dt| dt.to_string()),
        duration_ms: model.duration_ms,
    }
}

#[instrument_trace]
fn step_to_dto(model: mapper::cicd::step::Model) -> StepDto {
    let status: CicdStatus = model.status.parse().unwrap_or_default();
    StepDto {
        id: model.id,
        job_id: model.job_id,
        name: model.name,
        command: model.command,
        status,
        output: model.output,
        dockerfile: model.dockerfile,
        image: model.image,
        started_at: model.started_at.map(|dt| dt.to_string()),
        completed_at: model.completed_at.map(|dt| dt.to_string()),
        duration_ms: model.duration_ms,
    }
}
