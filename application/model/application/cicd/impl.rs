use super::*;

/// Implementation of `CicdStatus` for `fmt::Display`.
impl fmt::Display for CicdStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CicdStatus::Pending => write!(f, "pending"),
            CicdStatus::Running => write!(f, "running"),
            CicdStatus::Success => write!(f, "success"),
            CicdStatus::Failure => write!(f, "failure"),
            CicdStatus::Cancelled => write!(f, "cancelled"),
            CicdStatus::Skipped => write!(f, "skipped"),
        }
    }
}

/// Implementation of `CicdStatus` for `FromStr`.
impl FromStr for CicdStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(CicdStatus::Pending),
            "running" => Ok(CicdStatus::Running),
            "success" => Ok(CicdStatus::Success),
            "failure" => Ok(CicdStatus::Failure),
            "cancelled" => Ok(CicdStatus::Cancelled),
            "skipped" => Ok(CicdStatus::Skipped),
            _ => Ok(CicdStatus::default()),
        }
    }
}

/// Implementation of `TriggerType` for `fmt::Display`.
impl fmt::Display for TriggerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriggerType::Push => write!(f, "push"),
            TriggerType::PullRequest => write!(f, "pull_request"),
            TriggerType::Manual => write!(f, "manual"),
            TriggerType::Schedule => write!(f, "schedule"),
            TriggerType::Webhook => write!(f, "webhook"),
        }
    }
}

/// Implementation of `TriggerType` for `FromStr`.
impl FromStr for TriggerType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "push" => Ok(TriggerType::Push),
            "pull_request" => Ok(TriggerType::PullRequest),
            "manual" => Ok(TriggerType::Manual),
            "schedule" => Ok(TriggerType::Schedule),
            "webhook" => Ok(TriggerType::Webhook),
            _ => Ok(TriggerType::default()),
        }
    }
}

/// Implementation of methods for `CicdStatus`.
impl CicdStatus {
    /// Checks whether this status represents a terminal (completed) state.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the status is success, failure, cancelled, or skipped.
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            CicdStatus::Success | CicdStatus::Failure | CicdStatus::Cancelled | CicdStatus::Skipped
        )
    }

    /// Checks whether this status represents an actively running state.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the status is running.
    #[instrument_trace]
    pub fn is_active(self) -> bool {
        self == CicdStatus::Running
    }

    /// Checks whether this status represents a pending (waiting) state.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the status is pending.
    #[instrument_trace]
    pub fn is_pending(self) -> bool {
        self == CicdStatus::Pending
    }
}

/// Implementation of methods for `From`.
impl From<CicdStatus> for String {
    fn from(status: CicdStatus) -> String {
        status.to_string()
    }
}
