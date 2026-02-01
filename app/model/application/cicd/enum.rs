use super::*;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum CicdStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "skipped")]
    Skipped,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum TriggerType {
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "pull_request")]
    PullRequest,
    #[default]
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "webhook")]
    Webhook,
}
