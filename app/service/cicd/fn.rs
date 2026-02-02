use super::*;

#[instrument_trace]
pub fn pipeline_to_dto(model: Model) -> PipelineDto {
    PipelineDto {
        id: model.get_id(),
        name: model.get_name().clone(),
        description: model.try_get_description().clone(),
        config_content: model.try_get_config_content().clone(),
        created_at: model.try_get_created_at().as_ref().map(|dt| dt.to_string()),
        updated_at: model.try_get_updated_at().as_ref().map(|dt| dt.to_string()),
    }
}

#[instrument_trace]
pub fn run_to_dto(model: mapper::cicd::run::Model) -> RunDto {
    let status: CicdStatus = model.get_status().parse().unwrap_or_default();
    RunDto {
        id: model.get_id(),
        pipeline_id: model.get_pipeline_id(),
        pipeline_name: None,
        run_number: model.get_run_number(),
        status,
        triggered_by: model.try_get_triggered_by().clone(),
        commit_hash: model.try_get_commit_hash().clone(),
        commit_message: model.try_get_commit_message().clone(),
        started_at: model.try_get_started_at().as_ref().map(|dt| dt.to_string()),
        completed_at: model
            .try_get_completed_at()
            .as_ref()
            .map(|dt| dt.to_string()),
        duration_ms: model.get_duration_ms(),
        created_at: model.try_get_created_at().as_ref().map(|dt| dt.to_string()),
    }
}

#[instrument_trace]
pub fn job_to_dto(model: mapper::cicd::job::Model) -> JobDto {
    let status: CicdStatus = model.get_status().parse().unwrap_or_default();
    JobDto {
        id: model.get_id(),
        run_id: model.get_run_id(),
        name: model.get_name().clone(),
        status,
        runner: model.try_get_runner().clone(),
        started_at: model.try_get_started_at().as_ref().map(|dt| dt.to_string()),
        completed_at: model
            .try_get_completed_at()
            .as_ref()
            .map(|dt| dt.to_string()),
        duration_ms: model.get_duration_ms(),
    }
}

#[instrument_trace]
pub fn step_to_dto(model: mapper::cicd::step::Model) -> StepDto {
    let status: CicdStatus = model.get_status().parse().unwrap_or_default();
    StepDto {
        id: model.get_id(),
        job_id: model.get_job_id(),
        name: model.get_name().clone(),
        command: model.try_get_command().clone(),
        status,
        output: model.try_get_output().clone(),
        dockerfile: model.try_get_dockerfile().clone(),
        image: model.try_get_image().clone(),
        started_at: model.try_get_started_at().as_ref().map(|dt| dt.to_string()),
        completed_at: model
            .try_get_completed_at()
            .as_ref()
            .map(|dt| dt.to_string()),
        duration_ms: model.get_duration_ms(),
    }
}
