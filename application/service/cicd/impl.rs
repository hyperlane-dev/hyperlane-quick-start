use super::*;

impl From<CicdPipelineModel> for PipelineDto {
    fn from(model: CicdPipelineModel) -> Self {
        let mut dto: PipelineDto = Self::default();
        dto.set_id(model.get_id())
            .set_name(model.get_name().clone())
            .set_description(model.try_get_description().clone())
            .set_config_content(model.try_get_config_content().clone())
            .set_created_at(model.try_get_created_at().map(|dt| dt.to_string()))
            .set_updated_at(model.try_get_updated_at().map(|dt| dt.to_string()));
        dto
    }
}

impl From<CicdRunModel> for RunDto {
    fn from(model: CicdRunModel) -> Self {
        let status: CicdStatus = model.get_status().parse().unwrap_or_default();
        let mut dto: RunDto = Self::default();
        dto.set_id(model.get_id())
            .set_pipeline_id(model.get_pipeline_id())
            .set_pipeline_name(None)
            .set_run_number(model.get_run_number())
            .set_status(status)
            .set_triggered_by(model.try_get_triggered_by().clone())
            .set_commit_hash(model.try_get_commit_hash().clone())
            .set_commit_message(model.try_get_commit_message().clone())
            .set_started_at(model.try_get_started_at().map(|dt| dt.to_string()))
            .set_completed_at(model.try_get_completed_at().map(|dt| dt.to_string()))
            .set_duration_ms(model.get_duration_ms())
            .set_created_at(model.try_get_created_at().map(|dt| dt.to_string()));
        dto
    }
}

impl From<CicdJobModel> for JobDto {
    fn from(model: CicdJobModel) -> Self {
        let status: CicdStatus = model.get_status().parse().unwrap_or_default();
        let mut dto: JobDto = Self::default();
        dto.set_id(model.get_id())
            .set_run_id(model.get_run_id())
            .set_name(model.get_name().clone())
            .set_status(status)
            .set_runner(model.try_get_runner().clone())
            .set_started_at(model.try_get_started_at().map(|dt| dt.to_string()))
            .set_completed_at(model.try_get_completed_at().map(|dt| dt.to_string()))
            .set_duration_ms(model.get_duration_ms());
        dto
    }
}

impl From<CicdStepModel> for StepDto {
    fn from(model: CicdStepModel) -> Self {
        let status: CicdStatus = model.get_status().parse().unwrap_or_default();
        let mut dto: StepDto = Self::default();
        dto.set_id(model.get_id())
            .set_job_id(model.get_job_id())
            .set_name(model.get_name().clone())
            .set_command(model.try_get_command().clone())
            .set_status(status)
            .set_output(model.try_get_output().clone())
            .set_dockerfile(model.try_get_dockerfile().clone())
            .set_image(model.try_get_image().clone())
            .set_started_at(model.try_get_started_at().map(|dt| dt.to_string()))
            .set_completed_at(model.try_get_completed_at().map(|dt| dt.to_string()))
            .set_duration_ms(model.get_duration_ms());
        dto
    }
}

impl CicdService {
    #[instrument_trace]
    pub async fn create_pipeline(param: CreatePipelineParam) -> Result<i32, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let active_model: PipelineActiveModel = PipelineActiveModel::new(
            param.get_name().clone(),
            param.try_get_description().clone(),
            param.try_get_config_content().clone(),
        );
        let result: CicdPipelineModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.get_id())
    }

    #[instrument_trace]
    pub async fn get_pipeline_by_id(id: i32) -> Result<Option<PipelineDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<CicdPipelineModel> = PipelineEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.map(Into::into))
    }

    #[instrument_trace]
    pub async fn get_all_pipelines() -> Result<Vec<PipelineDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdPipelineModel> = PipelineEntity::find()
            .order_by_desc(PipelineColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    #[instrument_trace]
    pub async fn trigger_run(param: TriggerRunParam) -> Result<i32, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let pipeline_id: i32 = param.get_pipeline_id();
        let pipeline = Self::get_pipeline_by_id_with_config(pipeline_id).await?;
        let config_content: String = pipeline
            .and_then(|p| p.try_get_config_content().clone())
            .ok_or_else(|| "Pipeline config content is required".to_string())?;
        let run_number: i32 = Self::get_next_run_number(pipeline_id).await?;
        let active_model: RunActiveModel = RunActiveModel::new(
            pipeline_id,
            run_number,
            param.try_get_triggered_by().clone(),
            param.try_get_commit_hash().clone(),
            param.try_get_commit_message().clone(),
        );
        let run_result = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let run_id: i32 = run_result.get_id();
        Self::parse_config_and_create_jobs(&db, run_id, &config_content).await?;
        let run_id_clone: i32 = run_id;
        spawn(async move {
            if let Err(error) = Self::execute_run(run_id_clone).await {
                tracing::error!("Failed to execute run {}: {}", run_id_clone, error);
            }
        });
        Ok(run_id)
    }

    #[instrument_trace]
    async fn get_pipeline_by_id_with_config(id: i32) -> Result<Option<CicdPipelineModel>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let result: Option<CicdPipelineModel> = PipelineEntity::find_by_id(id)
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
        let pipeline_dockerfile: Option<String> = config.try_get_dockerfile().clone();
        let pipeline_image: Option<String> = config.try_get_image().clone();
        for (job_name, job_config) in config.get_jobs() {
            let job_dockerfile: Option<String> = job_config
                .try_get_dockerfile()
                .clone()
                .or_else(|| pipeline_dockerfile.clone());
            let job_image: Option<String> = job_config
                .try_get_image()
                .clone()
                .or_else(|| pipeline_image.clone());
            let job_active_model: mapper::cicd::job::ActiveModel =
                JobActiveModel::new(run_id, job_name.clone());
            let job_result = job_active_model
                .insert(db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            let job_id: i32 = job_result.get_id();
            for step_config in job_config.get_steps() {
                let step_dockerfile: Option<String> = step_config
                    .try_get_dockerfile()
                    .clone()
                    .or_else(|| job_dockerfile.clone());
                let step_image: Option<String> = job_image.clone();
                let step_active_model: mapper::cicd::step::ActiveModel =
                    if step_dockerfile.is_some() || step_image.is_some() {
                        StepActiveModel::new_with_dockerfile(
                            job_id,
                            step_config.get_name().clone(),
                            step_dockerfile,
                            step_image,
                        )
                    } else {
                        StepActiveModel::new(
                            job_id,
                            step_config.get_name().clone(),
                            step_config.try_get_run().clone(),
                        )
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
        if !is_docker_available().await {
            let _: Result<(), String> = Self::complete_run(run_id, CicdStatus::Failure).await;
            return Err("Docker is not installed or not available".to_string());
        }
        Self::start_run(run_id).await?;
        let jobs: Vec<JobDto> = Self::get_jobs_by_run(run_id).await?;
        let mut has_error: bool = false;
        for job in jobs {
            let job_id: i32 = job.get_id();
            let mut param: UpdateJobStatusParam = UpdateJobStatusParam::default();
            param
                .set_job_id(job_id)
                .set_status(CicdStatus::Running)
                .set_runner(Some("local-runner".to_string()));
            Self::update_job_status(param).await?;
            let steps: Vec<StepDto> = Self::get_steps_by_job(job_id).await?;
            let mut job_has_error: bool = false;
            for step in steps {
                let step_id: i32 = step.get_id();
                let log_manager: &LogStreamManager = get_log_stream_manager();
                log_manager.start_step_stream(run_id, step_id).await;
                let mut param: UpdateStepStatusParam = UpdateStepStatusParam::default();
                param
                    .set_step_id(step_id)
                    .set_status(CicdStatus::Running)
                    .set_output(None);
                Self::update_step_status(param).await?;
                let output: String =
                    if step.try_get_image().is_some() && step.try_get_dockerfile().is_none() {
                        Self::execute_image(run_id, &step).await
                    } else if step.try_get_dockerfile().is_some() {
                        Self::execute_dockerfile(run_id, &step).await
                    } else {
                        let command: String = step.try_get_command().clone().unwrap_or_default();
                        Self::execute_command(run_id, step_id, &command).await
                    };
                let step_status: CicdStatus = if output.starts_with("Error:") {
                    job_has_error = true;
                    has_error = true;
                    CicdStatus::Failure
                } else {
                    CicdStatus::Success
                };
                log_manager
                    .update_step_status(run_id, step_id, step_status)
                    .await;
                log_manager.end_step_stream(run_id, step_id).await;
                let mut param: UpdateStepStatusParam = UpdateStepStatusParam::default();
                param
                    .set_step_id(step_id)
                    .set_status(step_status)
                    .set_output(Some(output));
                Self::update_step_status(param).await?;
                if job_has_error {
                    break;
                }
            }
            let job_status: CicdStatus = if job_has_error {
                CicdStatus::Failure
            } else {
                CicdStatus::Success
            };
            let mut param: UpdateJobStatusParam = UpdateJobStatusParam::default();
            param
                .set_job_id(job_id)
                .set_status(job_status)
                .set_runner(Some("local-runner".to_string()));
            Self::update_job_status(param).await?;
            if job_has_error {
                break;
            }
        }
        let run_status: CicdStatus = if has_error {
            CicdStatus::Failure
        } else {
            CicdStatus::Success
        };
        let log_manager: &LogStreamManager = get_log_stream_manager();
        log_manager.end_run_streams(run_id).await;
        Self::complete_run(run_id, run_status).await?;
        Ok(())
    }

    #[instrument_trace]
    async fn execute_command(run_id: i32, step_id: i32, command: &str) -> String {
        if command.is_empty() {
            return "No command to execute".to_string();
        }
        let config: DockerConfig = DockerConfig::secure();
        let run_args: Vec<String> = Self::build_docker_args_for_cicd(&config, command);
        let container_id_output: Output =
            match Command::new("docker").args(&run_args).output().await {
                Ok(output) => output,
                Err(error) => {
                    return format!("Error: Failed to run Docker container: {error}");
                }
            };
        if !container_id_output.status.success() {
            let stderr: String = String::from_utf8_lossy(&container_id_output.stderr).to_string();
            return format!("Error: Failed to start container: {stderr}");
        }
        let container_id: String = String::from_utf8_lossy(&container_id_output.stdout)
            .trim()
            .to_string();
        let log_manager: &LogStreamManager = get_log_stream_manager();
        let output_result: Result<String, String> =
            Self::stream_container_logs(run_id, step_id, &container_id, log_manager).await;
        let _: Result<Output, std::io::Error> = Command::new("docker")
            .args(["rm", "-f", &container_id])
            .output()
            .await;
        match output_result {
            Ok(output) => output,
            Err(error) => format!("Error: {error}"),
        }
    }

    #[instrument_trace]
    async fn stream_container_logs(
        run_id: i32,
        step_id: i32,
        container_id: &str,
        log_manager: &LogStreamManager,
    ) -> Result<String, String> {
        let container_id: String = container_id.to_string();
        let log_manager: LogStreamManager = log_manager.clone();
        let stdout_handle: JoinHandle<Result<String, String>> = {
            let container_id: String = container_id.clone();
            let log_manager: LogStreamManager = log_manager.clone();
            spawn(async move {
                let child: Child = match Command::new("docker")
                    .args(["logs", "-f", &container_id])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .spawn()
                {
                    Ok(c) => c,
                    Err(error) => return Err(format!("Failed to spawn docker logs: {error}")),
                };
                Self::read_stdout_stream(child, run_id, step_id, &log_manager).await
            })
        };
        let stderr_handle: JoinHandle<Result<String, String>> = {
            let container_id: String = container_id.clone();
            let log_manager: LogStreamManager = log_manager.clone();
            spawn(async move {
                let child: Child = match Command::new("docker")
                    .args(["logs", "-f", &container_id])
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(c) => c,
                    Err(error) => return Err(format!("Failed to spawn docker logs: {error}")),
                };
                Self::read_stderr_stream(child, run_id, step_id, &log_manager).await
            })
        };
        let timeout_result: TimeoutResult = timeout(TASK_TIMEOUT, async {
            let stdout_result: StreamResult = stdout_handle.await;
            let stderr_result: StreamResult = stderr_handle.await;
            (stdout_result, stderr_result)
        })
        .await;
        let mut output_builder: StepOutputBuilder = StepOutputBuilder::new();
        let is_timeout: bool = timeout_result.is_err();
        if is_timeout {
            let _: Result<Output, std::io::Error> = Command::new("docker")
                .args(["stop", "-t", "10", &container_id])
                .output()
                .await;
            let logs_after_stop: Output = Command::new("docker")
                .args(["logs", &container_id])
                .output()
                .await
                .unwrap_or_else(|_| Output {
                    status: ExitStatus::default(),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            let stdout: String = String::from_utf8_lossy(&logs_after_stop.stdout).to_string();
            let stderr: String = String::from_utf8_lossy(&logs_after_stop.stderr).to_string();
            log_manager
                .append_log(run_id, step_id, &stdout, false)
                .await;
            log_manager.append_log(run_id, step_id, &stderr, true).await;
            output_builder.add_stdout(stdout);
            output_builder.add_stderr(stderr);
            output_builder.mark_timeout(TASK_TIMEOUT.as_secs());
        } else {
            let (stdout_result, stderr_result): StreamResultPair = match timeout_result {
                Ok(results) => results,
                Err(_) => (Ok(Ok(String::new())), Ok(Ok(String::new()))),
            };
            let stdout: String = stdout_result
                .unwrap_or(Ok(String::new()))
                .unwrap_or_default();
            let stderr: String = stderr_result
                .unwrap_or(Ok(String::new()))
                .unwrap_or_default();
            output_builder.add_stdout(stdout);
            output_builder.add_stderr(stderr);
        }
        Ok(output_builder.build())
    }

    #[instrument_trace]
    async fn read_stdout_stream(
        mut child: Child,
        run_id: i32,
        step_id: i32,
        log_manager: &LogStreamManager,
    ) -> Result<String, String> {
        let mut lines: Lines<BufReader<Pin<Box<dyn AsyncRead + Send>>>> = match child.stdout.take()
        {
            Some(s) => BufReader::new(Box::pin(s) as Pin<Box<dyn AsyncRead + Send>>),
            None => return Ok(String::new()),
        }
        .lines();
        let mut output_buffer: String = String::new();
        while let Ok(Some(line)) = lines.next_line().await {
            let line_with_newline: String = format!("{line}\n");
            log_manager
                .append_log(run_id, step_id, &line_with_newline, false)
                .await;
            output_buffer.push_str(&line_with_newline);
        }
        let _ = child.wait().await;
        Ok(output_buffer)
    }

    #[instrument_trace]
    async fn read_stderr_stream(
        mut child: Child,
        run_id: i32,
        step_id: i32,
        log_manager: &LogStreamManager,
    ) -> Result<String, String> {
        let mut lines: Lines<BufReader<Pin<Box<dyn AsyncRead + Send>>>> = match child.stderr.take()
        {
            Some(s) => BufReader::new(Box::pin(s) as Pin<Box<dyn AsyncRead + Send>>),
            None => return Ok(String::new()),
        }
        .lines();
        let mut output_buffer: String = String::new();
        while let Ok(Some(line)) = lines.next_line().await {
            let line_with_newline: String = format!("{line}\n");
            log_manager
                .append_log(run_id, step_id, &line_with_newline, true)
                .await;
            output_buffer.push_str(&line_with_newline);
        }
        let _ = child.wait().await;
        Ok(output_buffer)
    }

    #[instrument_trace]
    fn build_docker_args_for_cicd(config: &DockerConfig, command: &str) -> Vec<String> {
        let mut args: Vec<String> = vec!["run".to_string(), "-d".to_string()];
        if config.get_disable_network() {
            args.push("--network=none".to_string());
        }
        if let Some(cpus) = config.get_cpus() {
            args.push(format!("--cpus={cpus}"));
        }
        if let Some(memory) = config.try_get_memory() {
            args.push(format!("--memory={memory}"));
        }
        if let Some(pids_limit) = config.get_pids_limit() {
            args.push(format!("--pids-limit={pids_limit}"));
        }
        if config.get_read_only() {
            args.push("--read-only".to_string());
            args.push("--tmpfs".to_string());
            args.push("/tmp:rw,noexec,nosuid,size=100m".to_string());
        }
        args.push("-w".to_string());
        args.push(config.get_workdir().clone());
        args.push(config.get_image().clone());
        args.push(config.get_shell().clone());
        args.push(config.get_shell_flag().clone());
        args.push(command.to_string());
        args
    }

    #[instrument_trace]
    pub async fn execute_image(run_id: i32, step: &StepDto) -> String {
        let image: String = match step.try_get_image() {
            Some(img) => img.clone(),
            None => return "Error: No image specified".to_string(),
        };
        let step_id: i32 = step.get_id();
        let container_name: String = format!(
            "cicd-run-{run_id}-step-{step_id}-{}",
            Utc::now().timestamp()
        );
        let run_args: Vec<String> = vec![
            "run".to_string(),
            "-d".to_string(),
            "--name".to_string(),
            container_name.clone(),
            "--network=none".to_string(),
            "--cpus=1".to_string(),
            "--memory=512m".to_string(),
            "--pids-limit=100".to_string(),
            image.clone(),
        ];
        let container_id_output: Output =
            match Command::new("docker").args(&run_args).output().await {
                Ok(output) => output,
                Err(error) => {
                    return format!("Error: Failed to run Docker container: {error}");
                }
            };
        if !container_id_output.status.success() {
            let stderr: String = String::from_utf8_lossy(&container_id_output.stderr).to_string();
            return format!("Error: Failed to start container: {stderr}");
        }
        let container_id: String = String::from_utf8_lossy(&container_id_output.stdout)
            .trim()
            .to_string();
        let log_manager: &LogStreamManager = get_log_stream_manager();
        let output_result: Result<String, String> =
            Self::stream_container_logs(run_id, step_id, &container_id, log_manager).await;
        let _: Result<Output, std::io::Error> = Command::new("docker")
            .args(["rm", "-f", &container_id])
            .output()
            .await;
        match output_result {
            Ok(output) => output,
            Err(error) => format!("Error: {error}"),
        }
    }

    #[instrument_trace]
    pub async fn execute_dockerfile(run_id: i32, step: &StepDto) -> String {
        let dockerfile_content: String = match step.try_get_dockerfile() {
            Some(content) => content.clone(),
            None => return "Error: No Dockerfile content".to_string(),
        };
        let step_id: i32 = step.get_id();
        let image_tag: String = format!("cicd-run-{run_id}-step-{step_id}");
        let temp_dir: PathBuf = std::env::temp_dir().join(format!("cicd-{run_id}-{step_id}"));
        if let Err(error) = fs::create_dir_all(&temp_dir).await {
            return format!("Error: Failed to create temp directory: {error}");
        }
        let dockerfile_path: PathBuf = temp_dir.join("Dockerfile");
        if let Err(error) = fs::write(&dockerfile_path, dockerfile_content).await {
            return format!("Error: Failed to write Dockerfile: {error}");
        }
        let log_manager: &LogStreamManager = get_log_stream_manager();
        let build_log_header: String = "[Build Log]\n".to_string();
        log_manager
            .append_log(run_id, step_id, &build_log_header, false)
            .await;
        let build_result: Result<Result<Output, std::io::Error>, Elapsed> = timeout(
            TASK_TIMEOUT,
            Command::new("docker")
                .args([
                    "build",
                    "--progress=plain",
                    "-t",
                    &image_tag,
                    temp_dir.to_str().unwrap_or("."),
                ])
                .output(),
        )
        .await;
        let build_output: Output = match build_result {
            Ok(Ok(output)) => output,
            Ok(Err(error)) => {
                let _ = fs::remove_dir_all(&temp_dir).await;
                return format!("Error: Failed to build Docker image: {error}");
            }
            Err(_) => {
                let _ = fs::remove_dir_all(&temp_dir).await;
                return format!(
                    "Error: Docker build timeout after {} seconds",
                    TASK_TIMEOUT.as_secs()
                );
            }
        };
        let mut output_builder: StepOutputBuilder = StepOutputBuilder::new();
        let build_stdout: String = String::from_utf8_lossy(&build_output.stdout).to_string();
        let build_stderr: String = String::from_utf8_lossy(&build_output.stderr).to_string();
        log_manager
            .append_log(run_id, step_id, &build_stdout, false)
            .await;
        log_manager
            .append_log(run_id, step_id, &build_stderr, true)
            .await;
        output_builder.add_stdout(build_stdout.clone());
        output_builder.add_stderr(build_stderr.clone());
        if !build_output.status.success() {
            let _ = fs::remove_dir_all(&temp_dir).await;
            let _: Result<Output, std::io::Error> = Command::new("docker")
                .args(["rmi", &image_tag])
                .output()
                .await;
            return format!(
                "Error: Docker build failed\nStdout: {}\nStderr: {}",
                build_stdout.trim(),
                build_stderr.trim()
            );
        }
        let container_name: String = format!("cicd-run-{run_id}-step-{step_id}-run");
        let run_args: Vec<String> = vec![
            "run".to_string(),
            "-d".to_string(),
            "--name".to_string(),
            container_name.clone(),
            "--network=none".to_string(),
            "--cpus=1".to_string(),
            "--memory=512m".to_string(),
            "--pids-limit=100".to_string(),
            image_tag.clone(),
        ];
        let container_id_output: Output =
            match Command::new("docker").args(&run_args).output().await {
                Ok(output) => output,
                Err(error) => {
                    let _ = fs::remove_dir_all(&temp_dir).await;
                    let _: Result<Output, std::io::Error> = Command::new("docker")
                        .args(["rmi", &image_tag])
                        .output()
                        .await;
                    return format!("Error: Failed to run Docker container: {error}");
                }
            };
        if !container_id_output.status.success() {
            let stderr: String = String::from_utf8_lossy(&container_id_output.stderr).to_string();
            let _ = fs::remove_dir_all(&temp_dir).await;
            let _: Result<Output, std::io::Error> = Command::new("docker")
                .args(["rmi", &image_tag])
                .output()
                .await;
            return format!("Error: Failed to start container: {stderr}");
        }
        let container_id: String = String::from_utf8_lossy(&container_id_output.stdout)
            .trim()
            .to_string();
        let run_log_header: String = "\n[Run Output]\n".to_string();
        log_manager
            .append_log(run_id, step_id, &run_log_header, false)
            .await;
        let output_result: Result<String, String> =
            Self::stream_container_logs(run_id, step_id, &container_id, log_manager).await;
        let _: Result<Output, std::io::Error> = Command::new("docker")
            .args(["rm", "-f", &container_id])
            .output()
            .await;
        let _ = fs::remove_dir_all(&temp_dir).await;
        let _: Result<Output, std::io::Error> = Command::new("docker")
            .args(["rmi", &image_tag])
            .output()
            .await;
        match output_result {
            Ok(run_output) => {
                output_builder.add_stdout(run_output);
                output_builder.build()
            }
            Err(error) => format!("Error: {error}"),
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
        let result: Option<CicdRunModel> = RunEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.map(Into::into))
    }

    #[instrument_trace]
    pub async fn get_runs_by_pipeline(pipeline_id: i32) -> Result<Vec<RunDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdRunModel> = RunEntity::find()
            .filter(RunColumn::PipelineId.eq(pipeline_id))
            .order_by_desc(RunColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    #[instrument_trace]
    pub async fn query_runs(param: QueryRunsParam) -> Result<PaginatedRunsDto, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let page_size: i32 = param.try_get_page_size().unwrap_or(50);
        let mut query = RunEntity::find();
        if let Some(pipeline_id) = param.try_get_pipeline_id() {
            query = query.filter(RunColumn::PipelineId.eq(pipeline_id));
        }
        if let Some(status) = param.try_get_status() {
            query = query.filter(RunColumn::Status.eq(status.to_string()));
        }
        if let Some(last_id) = param.try_get_last_id() {
            query = query.filter(RunColumn::Id.lt(last_id));
        }
        let total: i32 = RunEntity::find()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i32;
        let models: Vec<CicdRunModel> = query
            .order_by_desc(RunColumn::Id)
            .limit((page_size + 1) as u64)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let has_more: bool = models.len() > page_size as usize;
        let runs: Vec<CicdRunModel> = if has_more {
            models.into_iter().take(page_size as usize).collect()
        } else {
            models
        };
        let mut dto = PaginatedRunsDto::default();
        dto.set_total(total)
            .set_runs(runs.into_iter().map(Into::into).collect())
            .set_has_more(has_more);
        Ok(dto)
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
            let started_at: NaiveDateTime = run_model.try_get_started_at().unwrap_or(now);
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
        let result: CicdJobModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.get_id())
    }

    #[instrument_trace]
    pub async fn get_jobs_by_run(run_id: i32) -> Result<Vec<JobDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdJobModel> = JobEntity::find()
            .filter(JobColumn::RunId.eq(run_id))
            .order_by_asc(JobColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    #[instrument_trace]
    pub async fn update_job_status(param: UpdateJobStatusParam) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        let param_status: CicdStatus = *param.get_status();
        let param_job_id: i32 = param.get_job_id();
        let param_runner: Option<String> = param.try_get_runner().clone();
        if param_status == CicdStatus::Running {
            JobEntity::update_many()
                .filter(JobColumn::Id.eq(param_job_id))
                .col_expr(JobColumn::Status, Expr::value(param_status.to_string()))
                .col_expr(JobColumn::Runner, Expr::value(param_runner))
                .col_expr(JobColumn::StartedAt, Expr::value(now))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        } else if param_status.is_terminal() {
            let job = JobEntity::find_by_id(param_job_id)
                .one(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            if let Some(job_model) = job {
                let started_at: NaiveDateTime =
                    job_model.try_get_started_at().map(|s| s).unwrap_or(now);
                let duration_ms: i32 = (now.and_utc().timestamp_millis()
                    - started_at.and_utc().timestamp_millis())
                    as i32;
                JobEntity::update_many()
                    .filter(JobColumn::Id.eq(param_job_id))
                    .col_expr(JobColumn::Status, Expr::value(param_status.to_string()))
                    .col_expr(JobColumn::CompletedAt, Expr::value(now))
                    .col_expr(JobColumn::DurationMs, Expr::value(duration_ms))
                    .exec(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
            }
        } else {
            JobEntity::update_many()
                .filter(JobColumn::Id.eq(param_job_id))
                .col_expr(JobColumn::Status, Expr::value(param_status.to_string()))
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
        let result: CicdStepModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.get_id())
    }

    #[instrument_trace]
    pub async fn get_steps_by_job(job_id: i32) -> Result<Vec<StepDto>, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let models: Vec<CicdStepModel> = StepEntity::find()
            .filter(StepColumn::JobId.eq(job_id))
            .order_by_asc(StepColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    #[instrument_trace]
    pub async fn update_step_status(param: UpdateStepStatusParam) -> Result<(), String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        let param_status: CicdStatus = *param.get_status();
        let param_step_id: i32 = param.get_step_id();
        let param_output: Option<String> = param.try_get_output().clone();
        if param_status == CicdStatus::Running {
            StepEntity::update_many()
                .filter(StepColumn::Id.eq(param_step_id))
                .col_expr(StepColumn::Status, Expr::value(param_status.to_string()))
                .col_expr(StepColumn::StartedAt, Expr::value(now))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        } else if param_status.is_terminal() {
            let step = StepEntity::find_by_id(param_step_id)
                .one(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            if let Some(step_model) = step {
                let started_at: NaiveDateTime = (*step_model.try_get_started_at()).unwrap_or(now);
                let duration_ms: i32 = (now.and_utc().timestamp_millis()
                    - started_at.and_utc().timestamp_millis())
                    as i32;
                StepEntity::update_many()
                    .filter(StepColumn::Id.eq(param_step_id))
                    .col_expr(StepColumn::Status, Expr::value(param_status.to_string()))
                    .col_expr(StepColumn::Output, Expr::value(param_output.clone()))
                    .col_expr(StepColumn::CompletedAt, Expr::value(now))
                    .col_expr(StepColumn::DurationMs, Expr::value(duration_ms))
                    .exec(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
            }
        } else {
            StepEntity::update_many()
                .filter(StepColumn::Id.eq(param_step_id))
                .col_expr(StepColumn::Status, Expr::value(param_status.to_string()))
                .col_expr(StepColumn::Output, Expr::value(param_output.clone()))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn get_run_detail(run_id: i32) -> Result<Option<RunDetailDto>, String> {
        let run: Option<RunDto> = Self::get_run_by_id(run_id).await?;
        if let Some(run_dto) = run {
            let jobs: Vec<JobDto> = Self::get_jobs_by_run(run_id).await?;
            let mut jobs_with_steps: Vec<JobWithStepsDto> = Vec::new();
            for job in jobs {
                let steps = Self::get_steps_by_job(job.get_id()).await?;
                let mut dto = JobWithStepsDto::default();
                dto.set_job(job).set_steps(steps);
                jobs_with_steps.push(dto);
            }
            let mut model = RunDetailDto::default();
            model.set_run(run_dto).set_jobs(jobs_with_steps);
            Ok(Some(model))
        } else {
            Ok(None)
        }
    }

    #[instrument_trace]
    pub async fn recover_interrupted_runs() -> Result<u32, String> {
        let db: DatabaseConnection =
            get_mysql_connection(DEFAULT_MYSQL_INSTANCE_NAME, None).await?;
        let running_runs: Vec<CicdRunModel> = RunEntity::find()
            .filter(RunColumn::Status.eq(CicdStatus::Running.to_string()))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let count: u32 = running_runs.len() as u32;
        if count == 0 {
            return Ok(0);
        }
        let now: NaiveDateTime = Utc::now().naive_utc();
        let error_message: &str = "[System] Task was interrupted due to server restart";
        for run in running_runs {
            let run_id: i32 = run.get_id();
            let jobs: Vec<CicdJobModel> = JobEntity::find()
                .filter(JobColumn::RunId.eq(run_id))
                .filter(JobColumn::Status.eq(CicdStatus::Running.to_string()))
                .all(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            for job in jobs {
                let job_id: i32 = job.get_id();
                let job_started_at: NaiveDateTime = job.try_get_started_at().unwrap_or(now);
                let job_duration_ms: i32 = (now.and_utc().timestamp_millis()
                    - job_started_at.and_utc().timestamp_millis())
                    as i32;
                JobEntity::update_many()
                    .filter(JobColumn::Id.eq(job_id))
                    .col_expr(
                        JobColumn::Status,
                        Expr::value(CicdStatus::Failure.to_string()),
                    )
                    .col_expr(JobColumn::CompletedAt, Expr::value(now))
                    .col_expr(JobColumn::DurationMs, Expr::value(job_duration_ms))
                    .exec(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
                let steps: Vec<CicdStepModel> = StepEntity::find()
                    .filter(StepColumn::JobId.eq(job_id))
                    .filter(StepColumn::Status.eq(CicdStatus::Running.to_string()))
                    .all(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
                for step in steps {
                    let step_id: i32 = step.get_id();
                    let step_started_at: NaiveDateTime =
                        step.try_get_started_at().map(|s| s).unwrap_or(now);
                    let step_duration_ms: i32 = (now.and_utc().timestamp_millis()
                        - step_started_at.and_utc().timestamp_millis())
                        as i32;
                    let step_output: String = step
                        .try_get_output()
                        .clone()
                        .map(|o| format!("{o}\n\n{error_message}"))
                        .unwrap_or_else(|| error_message.to_string());
                    StepEntity::update_many()
                        .filter(StepColumn::Id.eq(step_id))
                        .col_expr(
                            StepColumn::Status,
                            Expr::value(CicdStatus::Failure.to_string()),
                        )
                        .col_expr(StepColumn::Output, Expr::value(step_output))
                        .col_expr(StepColumn::CompletedAt, Expr::value(now))
                        .col_expr(StepColumn::DurationMs, Expr::value(step_duration_ms))
                        .exec(&db)
                        .await
                        .map_err(|error: DbErr| error.to_string())?;
                }
            }
            let started_at: NaiveDateTime = run.try_get_started_at().unwrap_or(now);
            let duration_ms: i32 =
                (now.and_utc().timestamp_millis() - started_at.and_utc().timestamp_millis()) as i32;
            RunEntity::update_many()
                .filter(RunColumn::Id.eq(run_id))
                .col_expr(
                    RunColumn::Status,
                    Expr::value(CicdStatus::Failure.to_string()),
                )
                .col_expr(RunColumn::CompletedAt, Expr::value(now))
                .col_expr(RunColumn::DurationMs, Expr::value(duration_ms))
                .exec(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            tracing::warn!(
                "Run {} was interrupted by server restart and marked as failed",
                run_id
            );
        }
        Ok(count)
    }

    #[instrument_trace]
    pub async fn get_incremental_run_detail(
        run_id: i32,
        step_offsets: Vec<StepOffsetParam>,
    ) -> Result<Option<IncrementalRunDetailDto>, String> {
        let run: Option<RunDto> = Self::get_run_by_id(run_id).await?;
        if let Some(run_dto) = run {
            let jobs: Vec<JobDto> = Self::get_jobs_by_run(run_id).await?;
            let mut jobs_with_steps: Vec<JobWithIncrementalStepsDto> = Vec::new();
            let log_manager: &LogStreamManager = get_log_stream_manager();
            for job in jobs {
                let steps: Vec<StepDto> = Self::get_steps_by_job(job.get_id()).await?;
                let mut step_logs: Vec<StepLogDto> = Vec::new();
                for step in steps {
                    let step_id: i32 = step.get_id();
                    let offset: usize = step_offsets
                        .iter()
                        .find(|o: &&StepOffsetParam| o.get_step_id() == step_id)
                        .map(|o: &StepOffsetParam| o.get_offset())
                        .unwrap_or(0);
                    let stderr_offset: usize = step_offsets
                        .iter()
                        .find(|o: &&StepOffsetParam| o.get_step_id() == step_id)
                        .map(|o: &StepOffsetParam| o.get_stderr_offset())
                        .unwrap_or(0);
                    let stdout_opt: Option<String> =
                        log_manager.get_step_stdout(run_id, step_id).await;
                    let stderr_opt: Option<String> =
                        log_manager.get_step_stderr(run_id, step_id).await;
                    let (stdout_output, stderr_output): (Option<String>, Option<String>) =
                        match (stdout_opt, stderr_opt) {
                            (Some(stdout), Some(stderr)) => (Some(stdout), Some(stderr)),
                            (Some(stdout), None) => (Some(stdout), None),
                            (None, Some(stderr)) => (None, Some(stderr)),
                            (None, None) => {
                                let db_output: Option<String> = step.try_get_output().clone();
                                (db_output.clone(), None)
                            }
                        };
                    let stdout_str: &str = stdout_output.as_deref().unwrap_or("");
                    let stderr_str: &str = stderr_output.as_deref().unwrap_or("");
                    let stdout_length: usize = stdout_str.len();
                    let stderr_length: usize = stderr_str.len();
                    let (new_stdout, final_stdout_offset): (Option<String>, usize) =
                        if offset < stdout_length {
                            let new_content: String = stdout_str[offset..].to_string();
                            (Some(new_content), stdout_length)
                        } else {
                            (None, stdout_length)
                        };
                    let (new_stderr, final_stderr_offset): (Option<String>, usize) =
                        if stderr_offset < stderr_length {
                            let new_content: String = stderr_str[stderr_offset..].to_string();
                            (Some(new_content), stderr_length)
                        } else {
                            (None, stderr_length)
                        };
                    let mut log_dto = StepLogDto::default();
                    log_dto
                        .set_step_id(step_id)
                        .set_step_name(step.get_name().clone())
                        .set_status(*step.get_status())
                        .set_output(stdout_output)
                        .set_output_length(final_stdout_offset)
                        .set_new_output(new_stdout)
                        .set_output_offset(offset)
                        .set_stderr_output(stderr_output)
                        .set_stderr_length(final_stderr_offset)
                        .set_new_stderr(new_stderr)
                        .set_stderr_offset(stderr_offset);
                    step_logs.push(log_dto);
                }
                let mut job_dto = JobWithIncrementalStepsDto::default();
                job_dto.set_job(job).set_steps(step_logs);
                jobs_with_steps.push(job_dto);
            }
            let mut detail_dto = IncrementalRunDetailDto::default();
            detail_dto.set_run(run_dto).set_jobs(jobs_with_steps);
            Ok(Some(detail_dto))
        } else {
            Ok(None)
        }
    }
}

impl StepOutputBuilder {
    fn new() -> Self {
        Self {
            stdout: String::new(),
            stderr: String::new(),
            is_timeout: false,
            timeout_secs: 0,
        }
    }

    fn add_stdout<C>(&mut self, content: C)
    where
        C: AsRef<str>,
    {
        self.stdout.push_str(content.as_ref());
    }

    fn add_stderr<C>(&mut self, content: C)
    where
        C: AsRef<str>,
    {
        self.stderr.push_str(content.as_ref());
    }

    fn mark_timeout(&mut self, secs: u64) {
        self.is_timeout = true;
        self.timeout_secs = secs;
    }

    fn build(self) -> String {
        let mut parts: Vec<String> = Vec::new();
        let stdout: String = self.stdout.trim().to_string();
        let stderr: String = self.stderr.trim().to_string();
        if !stdout.is_empty() {
            parts.push(format!("[Stdout]\n{stdout}"));
        }
        if !stderr.is_empty() {
            parts.push(format!("[Stderr]\n{stderr}"));
        }
        if self.is_timeout {
            parts.push(format!(
                "[Timeout]\nTask was cancelled after {} seconds due to timeout",
                self.timeout_secs
            ));
        }
        if parts.is_empty() {
            "Command executed successfully (no output)".to_string()
        } else if self.is_timeout {
            format!("Error: Task timeout\n\n{}", parts.join("\n\n"))
        } else {
            parts.join("\n\n")
        }
    }
}

impl LogStreamManager {
    #[instrument_trace]
    pub fn new() -> Self {
        Self {
            broadcast_map: Arc::new(BroadcastMap::new()),
            step_outputs: arc_rwlock(HashMap::new()),
            step_statuses: arc_rwlock(HashMap::new()),
            active_steps: arc_rwlock(HashMap::new()),
        }
    }

    #[instrument_trace]
    fn get_step_key(run_id: i32, step_id: i32) -> String {
        format!("{run_id}:{step_id}")
    }

    #[instrument_trace]
    pub async fn start_step_stream(&self, run_id: i32, step_id: i32) {
        let key: String = Self::get_step_key(run_id, step_id);
        let _receiver: BroadcastMapReceiver<String> = self
            .broadcast_map
            .subscribe_or_insert(key, DEFAULT_BROADCAST_SENDER_CAPACITY);
        let mut outputs = self.get_step_outputs().write().await;
        outputs.insert(
            step_id,
            StepOutput {
                stdout: arc_rwlock(String::new()),
                stderr: arc_rwlock(String::new()),
            },
        );
        self.step_statuses
            .write()
            .await
            .insert(step_id, arc_rwlock(CicdStatus::Running));
        self.get_active_steps()
            .write()
            .await
            .entry(run_id)
            .or_insert_with(HashSet::new)
            .insert(step_id);
    }

    #[instrument_trace]
    pub async fn append_log(&self, run_id: i32, step_id: i32, content: &str, is_stderr: bool) {
        let key: String = Self::get_step_key(run_id, step_id);
        let entry: LogEntry = LogEntry {
            step_id,
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            is_stderr,
        };
        let entry_json: String = serde_json::to_string(&entry).unwrap_or_default();
        let _ = self.broadcast_map.send(key, entry_json);
        if let Some(output) = self.get_step_outputs().read().await.get(&step_id) {
            if is_stderr {
                let mut stderr_guard: RwLockWriteGuard<'_, String> = output.stderr.write().await;
                stderr_guard.push_str(content);
            } else {
                let mut stdout_guard: RwLockWriteGuard<'_, String> = output.stdout.write().await;
                stdout_guard.push_str(content);
            }
        }
    }

    #[instrument_trace]
    pub async fn create_step_receiver(
        &self,
        run_id: i32,
        step_id: i32,
    ) -> Option<BroadcastMapReceiver<String>> {
        let key: String = Self::get_step_key(run_id, step_id);
        self.broadcast_map.subscribe(key)
    }

    #[instrument_trace]
    pub async fn update_step_status(&self, _run_id: i32, step_id: i32, status: CicdStatus) {
        if let Some(step_status) = self.step_statuses.read().await.get(&step_id) {
            let mut status_guard: RwLockWriteGuard<'_, CicdStatus> = step_status.write().await;
            *status_guard = status;
        }
    }

    #[instrument_trace]
    pub async fn get_step_output(&self, _run_id: i32, step_id: i32) -> Option<String> {
        let step_outputs: RwLockReadGuard<'_, HashMap<i32, StepOutput>> =
            self.get_step_outputs().read().await;
        let output: &StepOutput = step_outputs.get(&step_id)?;
        let stdout_guard: RwLockReadGuard<'_, String> = output.get_stdout().read().await;
        let stderr_guard: RwLockReadGuard<'_, String> = output.stderr.read().await;
        let mut result: String = String::new();
        if !stdout_guard.is_empty() {
            result.push_str("[Stdout]\n");
            result.push_str(&stdout_guard);
        }
        if !stderr_guard.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str("[Stderr]\n");
            result.push_str(&stderr_guard);
        }
        Some(result)
    }

    #[instrument_trace]
    pub async fn get_step_stdout(&self, _run_id: i32, step_id: i32) -> Option<String> {
        Some(
            self.get_step_outputs()
                .read()
                .await
                .get(&step_id)?
                .stdout
                .read()
                .await
                .clone(),
        )
    }

    #[instrument_trace]
    pub async fn get_step_stderr(&self, _run_id: i32, step_id: i32) -> Option<String> {
        Some(
            self.get_step_outputs()
                .read()
                .await
                .get(&step_id)?
                .stderr
                .read()
                .await
                .clone(),
        )
    }

    #[instrument_trace]
    pub async fn get_run_step_ids(&self, run_id: i32) -> Vec<i32> {
        self.get_active_steps()
            .read()
            .await
            .get(&run_id)
            .map(|steps| steps.iter().copied().collect())
            .unwrap_or_default()
    }

    #[instrument_trace]
    pub async fn end_step_stream(&self, run_id: i32, step_id: i32) {
        if let Some(steps) = self.get_active_steps().write().await.get_mut(&run_id) {
            steps.remove(&step_id);
        }
    }

    #[instrument_trace]
    pub async fn end_run_streams(&self, run_id: i32) {
        self.get_active_steps().write().await.remove(&run_id);
    }
}

impl Default for LogStreamManager {
    #[instrument_trace]
    fn default() -> Self {
        Self::new()
    }
}
