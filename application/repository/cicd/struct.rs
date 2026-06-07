use super::*;

/// Repository for performing database operations on the `cicd_pipeline` table.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct PipelineRepository;

/// Repository for performing database operations on the `cicd_run` table.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RunRepository;

/// Repository for performing database operations on the `cicd_job` table.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct JobRepository;

/// Repository for performing database operations on the `cicd_step` table.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct StepRepository;
