use super::*;

#[instrument_trace]
pub fn pipeline_to_dto(model: Model) -> PipelineDto {
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
pub fn run_to_dto(model: mapper::cicd::run::Model) -> RunDto {
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
pub fn job_to_dto(model: mapper::cicd::job::Model) -> JobDto {
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
pub fn step_to_dto(model: mapper::cicd::step::Model) -> StepDto {
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
