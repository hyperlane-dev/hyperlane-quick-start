use super::*;

/// One pipeline of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct Pipeline {
    /// The identifier of the pipeline.
    pub id: i32,
    /// The name of the pipeline.
    pub name: String,
    /// The description of the pipeline.
    pub description: Option<String>,
    /// The configuration of the pipeline.
    pub config_content: Option<String>,
    /// The moment the pipeline was created, in milliseconds since the Unix epoch.
    pub created_at: Option<i64>,
    /// The moment the pipeline was updated, in milliseconds since the Unix epoch.
    pub updated_at: Option<i64>,
}

/// One run of a pipeline.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct Run {
    /// The identifier of the run.
    pub id: i32,
    /// The identifier of the pipeline the run belongs to.
    pub pipeline_id: i32,
    /// The name of the pipeline the run belongs to.
    pub pipeline_name: Option<String>,
    /// The number of the run inside its pipeline.
    pub run_number: i32,
    /// The state of the run.
    pub status: CicdStatus,
    /// The visitor that triggered the run.
    pub triggered_by: Option<String>,
    /// The commit the run was built from.
    pub commit_hash: Option<String>,
    /// The message of the commit the run was built from.
    pub commit_message: Option<String>,
    /// The moment the run started, in milliseconds since the Unix epoch.
    pub started_at: Option<i64>,
    /// The moment the run finished, in milliseconds since the Unix epoch.
    pub completed_at: Option<i64>,
    /// The duration of the run in milliseconds.
    pub duration_ms: i32,
    /// The moment the run was created, in milliseconds since the Unix epoch.
    pub created_at: Option<i64>,
}

/// One job of a run.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct Job {
    /// The identifier of the job.
    pub id: i32,
    /// The identifier of the run the job belongs to.
    pub run_id: i32,
    /// The name of the job.
    pub name: String,
    /// The state of the job.
    pub status: CicdStatus,
    /// The runner that carried the job out.
    pub runner: Option<String>,
    /// The moment the job started, in milliseconds since the Unix epoch.
    pub started_at: Option<i64>,
    /// The moment the job finished, in milliseconds since the Unix epoch.
    pub completed_at: Option<i64>,
    /// The duration of the job in milliseconds.
    pub duration_ms: i32,
}

/// One step of a job.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct Step {
    /// The identifier of the step.
    pub id: i32,
    /// The identifier of the job the step belongs to.
    pub job_id: i32,
    /// The name of the step.
    pub name: String,
    /// The command the step carries out.
    pub command: Option<String>,
    /// The state of the step.
    pub status: CicdStatus,
    /// The output the step wrote.
    pub output: Option<String>,
    /// The moment the step started, in milliseconds since the Unix epoch.
    pub started_at: Option<i64>,
    /// The moment the step finished, in milliseconds since the Unix epoch.
    pub completed_at: Option<i64>,
    /// The duration of the step in milliseconds.
    pub duration_ms: i32,
}

/// One job of a run together with the steps it holds.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct JobWithSteps {
    /// The job.
    pub job: Job,
    /// The steps of the job.
    pub steps: Vec<Step>,
}

/// The detail of one run together with the jobs it holds.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct RunDetail {
    /// The run.
    pub run: Run,
    /// The jobs of the run.
    pub jobs: Vec<JobWithSteps>,
}

/// One page of the runs of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct RunPage {
    /// The number of the runs the server holds in total.
    pub total: i32,
    /// The runs of the page.
    pub runs: Vec<Run>,
    /// Whether the server holds older runs.
    pub has_more: bool,
}

/// One line of the log stream of a run.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct LogEntry {
    /// The identifier of the step the line belongs to.
    pub step_id: i32,
    /// The moment the line was written, in milliseconds since the Unix epoch.
    pub timestamp: i64,
    /// Whether the line was written to the error stream.
    pub is_stderr: bool,
    /// The text of the line.
    pub content: String,
}

/// The body of a pipeline the visitor creates.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreatePipelineBody {
    /// The name of the pipeline.
    pub name: String,
    /// The description of the pipeline.
    pub description: Option<String>,
    /// The configuration of the pipeline.
    pub config_content: Option<String>,
}

/// The body of a run the visitor triggers.
#[derive(Clone, Debug, Default, Serialize)]
pub struct TriggerRunBody {
    /// The identifier of the pipeline the run belongs to.
    pub pipeline_id: i32,
    /// The visitor that triggered the run.
    pub triggered_by: Option<String>,
}
