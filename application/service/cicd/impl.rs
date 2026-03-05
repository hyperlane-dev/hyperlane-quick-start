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
            .set_started_at(model.try_get_started_at().map(|dt| dt.to_string()))
            .set_completed_at(model.try_get_completed_at().map(|dt| dt.to_string()))
            .set_duration_ms(model.get_duration_ms());
        dto
    }
}

impl CicdService {
    #[instrument_trace]
    pub async fn create_pipeline(param: CreatePipelineParam) -> Result<i32, String> {
        PipelineRepository::create(
            param.get_name().clone(),
            param.try_get_description().clone(),
            param.try_get_config_content().clone(),
        )
        .await
    }

    #[instrument_trace]
    pub async fn get_pipeline_by_id(id: i32) -> Result<Option<PipelineDto>, String> {
        let result: Option<CicdPipelineModel> = PipelineRepository::find_by_id(id).await?;
        Ok(result.map(Into::into))
    }

    #[instrument_trace]
    pub async fn get_all_pipelines() -> Result<Vec<PipelineDto>, String> {
        let models: Vec<CicdPipelineModel> = PipelineRepository::find_all().await?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    #[instrument_trace]
    pub async fn trigger_run(param: TriggerRunParam) -> Result<i32, String> {
        let pipeline_id: i32 = param.get_pipeline_id();
        let pipeline: Option<CicdPipelineModel> =
            PipelineRepository::find_by_id(pipeline_id).await?;
        let config_content: String = pipeline
            .and_then(|p| p.try_get_config_content().clone())
            .ok_or_else(|| "Pipeline config content is required".to_string())?;
        let run_number: i32 = RunRepository::get_next_run_number(pipeline_id).await?;
        let run_result: CicdRunModel = RunRepository::create(
            pipeline_id,
            run_number,
            param.try_get_triggered_by().clone(),
            param.try_get_commit_hash().clone(),
            param.try_get_commit_message().clone(),
        )
        .await?;
        let run_id: i32 = run_result.get_id();
        Self::parse_config_and_create_jobs(run_id, &config_content).await?;
        let run_id_clone: i32 = run_id;
        spawn(async move {
            if let Err(error) = Self::execute_run(run_id_clone).await {
                tracing::error!("Failed to execute run {}: {}", run_id_clone, error);
            }
        });
        Ok(run_id)
    }

    #[instrument_trace]
    async fn parse_config_and_create_jobs(run_id: i32, config_content: &str) -> Result<(), String> {
        let config: PipelineConfig = serde_yaml::from_str(config_content)
            .map_err(|error| format!("Failed to parse config: {error}"))?;
        for (job_name, job_config) in config.get_jobs() {
            let job_result: CicdJobModel = JobRepository::create(run_id, job_name.clone()).await?;
            let job_id: i32 = job_result.get_id();
            for step_config in job_config.get_steps() {
                StepRepository::create(
                    job_id,
                    step_config.get_name().clone(),
                    step_config.try_get_run().clone(),
                )
                .await?;
            }
        }
        Ok(())
    }

    #[instrument_trace]
    pub async fn execute_run(run_id: i32) -> Result<(), String> {
        RunRepository::start(run_id).await?;
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
                let command: String = step.try_get_command().clone().unwrap_or_default();
                let output: String = Self::execute_command(run_id, step_id, &command).await;
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
        RunRepository::complete(run_id, run_status).await?;
        Ok(())
    }

    #[instrument_trace]
    async fn execute_command(run_id: i32, step_id: i32, command: &str) -> String {
        if command.is_empty() {
            return "No command to execute".to_string();
        }
        let log_manager: &LogStreamManager = get_log_stream_manager();
        let output_result: Result<String, String> =
            Self::execute_shell_command(run_id, step_id, command, log_manager).await;
        match output_result {
            Ok(output) => output,
            Err(error) => format!("Error: {error}"),
        }
    }

    #[instrument_trace]
    async fn execute_shell_command(
        run_id: i32,
        step_id: i32,
        command: &str,
        log_manager: &LogStreamManager,
    ) -> Result<String, String> {
        let is_windows: bool = cfg!(target_os = "windows");
        let shell: String = if is_windows {
            std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string())
        };
        let mut cmd: Command = Command::new(&shell);
        if is_windows {
            cmd.arg("/C").arg(command);
        } else {
            cmd.arg("-c").arg(command);
        }
        let mut child: Child = match cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
            Ok(c) => c,
            Err(error) => return Err(format!("Failed to spawn shell process: {error}")),
        };
        let stdout: ChildStdout = child
            .stdout
            .take()
            .ok_or_else(|| "Failed to take stdout".to_string())?;
        let stderr: ChildStderr = child
            .stderr
            .take()
            .ok_or_else(|| "Failed to take stderr".to_string())?;
        let stdout_handle: JoinHandle<Result<String, String>> = {
            let log_manager: LogStreamManager = log_manager.clone();
            spawn(
                async move { Self::read_stdout_stream(stdout, run_id, step_id, &log_manager).await },
            )
        };
        let stderr_handle: JoinHandle<Result<String, String>> = {
            let log_manager: LogStreamManager = log_manager.clone();
            spawn(
                async move { Self::read_stderr_stream(stderr, run_id, step_id, &log_manager).await },
            )
        };
        let timeout_result: Result<
            (StreamResult, StreamResult, std::io::Result<ExitStatus>),
            Elapsed,
        > = timeout(TASK_TIMEOUT, async move {
            let stdout_result: StreamResult = stdout_handle.await;
            let stderr_result: StreamResult = stderr_handle.await;
            let exit_status: std::io::Result<ExitStatus> = child.wait().await;
            (stdout_result, stderr_result, exit_status)
        })
        .await;
        let mut output_builder: StepOutputBuilder = StepOutputBuilder::new();
        match timeout_result {
            Ok((stdout_result, stderr_result, exit_status)) => {
                let stdout: String = stdout_result
                    .unwrap_or(Ok(String::new()))
                    .unwrap_or_default();
                let stderr: String = stderr_result
                    .unwrap_or(Ok(String::new()))
                    .unwrap_or_default();
                output_builder.add_stdout(stdout);
                output_builder.add_stderr(stderr);
                if let Ok(status) = exit_status
                    && !status.success()
                {
                    let exit_code: i32 = status.code().unwrap_or(-1);
                    return Err(format!("Command exited with code {exit_code}"));
                }
            }
            Err(_) => {
                output_builder.mark_timeout(TASK_TIMEOUT.as_secs());
            }
        }
        Ok(output_builder.build())
    }

    #[instrument_trace]
    async fn read_stdout_stream(
        reader: ChildStdout,
        run_id: i32,
        step_id: i32,
        log_manager: &LogStreamManager,
    ) -> Result<String, String> {
        let mut reader: ChildStdout = reader;
        let mut output_buffer: Vec<u8> = Vec::new();
        match reader.read_to_end(&mut output_buffer).await {
            Ok(_) => {
                let output: String = String::from_utf8_lossy(&output_buffer).to_string();
                if !output.is_empty() {
                    log_manager
                        .append_log(run_id, step_id, &output, false)
                        .await;
                }
                Ok(output)
            }
            Err(error) => Err(format!("Failed to read stdout: {error}")),
        }
    }

    #[instrument_trace]
    async fn read_stderr_stream(
        reader: ChildStderr,
        run_id: i32,
        step_id: i32,
        log_manager: &LogStreamManager,
    ) -> Result<String, String> {
        let mut reader: ChildStderr = reader;
        let mut output_buffer: Vec<u8> = Vec::new();
        match reader.read_to_end(&mut output_buffer).await {
            Ok(_) => {
                let output: String = String::from_utf8_lossy(&output_buffer).to_string();
                if !output.is_empty() {
                    log_manager.append_log(run_id, step_id, &output, true).await;
                }
                Ok(output)
            }
            Err(error) => Err(format!("Failed to read stderr: {error}")),
        }
    }

    #[instrument_trace]
    pub async fn get_next_run_number(pipeline_id: i32) -> Result<i32, String> {
        RunRepository::get_next_run_number(pipeline_id).await
    }

    #[instrument_trace]
    pub async fn get_run_by_id(id: i32) -> Result<Option<RunDto>, String> {
        let result: Option<CicdRunModel> = RunRepository::find_by_id(id).await?;
        Ok(result.map(Into::into))
    }

    #[instrument_trace]
    pub async fn get_runs_by_pipeline(pipeline_id: i32) -> Result<Vec<RunDto>, String> {
        let models: Vec<CicdRunModel> = RunRepository::find_by_pipeline(pipeline_id).await?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    #[instrument_trace]
    pub async fn query_runs(param: QueryRunsParam) -> Result<PaginatedRunsDto, String> {
        let page_size: u64 = param.try_get_page_size().unwrap_or(50) as u64;
        let status_str: Option<String> = param.try_get_status().map(|s| s.to_string());
        let (models, total, has_more): (Vec<CicdRunModel>, i32, bool) =
            RunRepository::query_with_pagination(
                param.try_get_pipeline_id(),
                status_str,
                param.try_get_last_id(),
                page_size + 1,
            )
            .await?;
        let mut dto = PaginatedRunsDto::default();
        dto.set_total(total)
            .set_runs(models.into_iter().map(Into::into).collect())
            .set_has_more(has_more);
        Ok(dto)
    }

    #[instrument_trace]
    pub async fn update_run_status(id: i32, status: CicdStatus) -> Result<(), String> {
        RunRepository::update_status(id, status).await
    }

    #[instrument_trace]
    pub async fn start_run(id: i32) -> Result<(), String> {
        RunRepository::start(id).await
    }

    #[instrument_trace]
    pub async fn complete_run(id: i32, status: CicdStatus) -> Result<(), String> {
        RunRepository::complete(id, status).await
    }

    #[instrument_trace]
    pub async fn create_job(run_id: i32, name: String) -> Result<i32, String> {
        let result: CicdJobModel = JobRepository::create(run_id, name).await?;
        Ok(result.get_id())
    }

    #[instrument_trace]
    pub async fn get_jobs_by_run(run_id: i32) -> Result<Vec<JobDto>, String> {
        let models: Vec<CicdJobModel> = JobRepository::find_by_run(run_id).await?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    #[instrument_trace]
    pub async fn update_job_status(param: UpdateJobStatusParam) -> Result<(), String> {
        JobRepository::update_status(
            param.get_job_id(),
            *param.get_status(),
            param.try_get_runner().clone(),
        )
        .await
    }

    #[instrument_trace]
    pub async fn create_step(
        job_id: i32,
        name: String,
        command: Option<String>,
    ) -> Result<i32, String> {
        let result: CicdStepModel = StepRepository::create(job_id, name, command).await?;
        Ok(result.get_id())
    }

    #[instrument_trace]
    pub async fn get_steps_by_job(job_id: i32) -> Result<Vec<StepDto>, String> {
        let models: Vec<CicdStepModel> = StepRepository::find_by_job(job_id).await?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    #[instrument_trace]
    pub async fn update_step_status(param: UpdateStepStatusParam) -> Result<(), String> {
        StepRepository::update_status(
            param.get_step_id(),
            *param.get_status(),
            param.try_get_output().clone(),
        )
        .await
    }

    #[instrument_trace]
    pub async fn get_run_detail(run_id: i32) -> Result<Option<RunDetailDto>, String> {
        let run: Option<RunDto> = Self::get_run_by_id(run_id).await?;
        if let Some(run_dto) = run {
            let jobs: Vec<JobDto> = Self::get_jobs_by_run(run_id).await?;
            let mut jobs_with_steps: Vec<JobWithStepsDto> = Vec::new();
            for job in jobs {
                let steps: Vec<StepDto> = Self::get_steps_by_job(job.get_id()).await?;
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
        let running_runs: Vec<CicdRunModel> =
            RunRepository::find_by_status(CicdStatus::Running).await?;
        let count: u32 = running_runs.len() as u32;
        if count == 0 {
            return Ok(0);
        }
        let error_message: &str = "[System] Task was interrupted due to server restart";
        for run in running_runs {
            let run_id: i32 = run.get_id();
            let jobs: Vec<CicdJobModel> =
                JobRepository::find_by_run_and_status(run_id, CicdStatus::Running).await?;
            for job in jobs {
                let job_id: i32 = job.get_id();
                JobRepository::update_status(
                    job_id,
                    CicdStatus::Failure,
                    job.try_get_runner().clone(),
                )
                .await?;
                let steps: Vec<CicdStepModel> =
                    StepRepository::find_by_job_and_status(job_id, CicdStatus::Running).await?;
                for step in steps {
                    let step_id: i32 = step.get_id();
                    let step_output: String = step
                        .try_get_output()
                        .clone()
                        .map(|o| format!("{o}\n\n{error_message}"))
                        .unwrap_or_else(|| error_message.to_string());
                    StepRepository::update_status(step_id, CicdStatus::Failure, Some(step_output))
                        .await?;
                }
            }
            RunRepository::complete(run_id, CicdStatus::Failure).await?;
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
