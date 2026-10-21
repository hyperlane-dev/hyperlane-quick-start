use super::*;

/// The state of one job, one step or one run of a pipeline.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum CicdStatus {
    /// The work has not started yet.
    #[default]
    #[serde(rename = "pending")]
    Pending,
    /// The work is running.
    #[serde(rename = "running")]
    Running,
    /// The work finished successfully.
    #[serde(rename = "success")]
    Success,
    /// The work failed.
    #[serde(rename = "failure")]
    Failure,
    /// The work was cancelled.
    #[serde(rename = "cancelled")]
    Cancelled,
    /// The work was skipped.
    #[serde(rename = "skipped")]
    Skipped,
}
