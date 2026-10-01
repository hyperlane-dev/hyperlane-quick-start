use super::*;

/// Implementation of `CicdStatus` for `fmt::Display`.
impl fmt::Display for CicdStatus {
    /// Formats the value for display.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The output formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The fmt result.
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
    /// Parses the value from its textual form.
    ///
    /// # Arguments
    ///
    /// - `&str` - The textual input.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The converted str, or the failure reason.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            STATUS_PENDING => Ok(CicdStatus::Pending),
            STATUS_RUNNING => Ok(CicdStatus::Running),
            STATUS_SUCCESS => Ok(CicdStatus::Success),
            STATUS_FAILURE => Ok(CicdStatus::Failure),
            STATUS_CANCELLED => Ok(CicdStatus::Cancelled),
            STATUS_SKIPPED => Ok(CicdStatus::Skipped),
            _ => Ok(CicdStatus::default()),
        }
    }
}

/// Implementation of `TriggerType` for `fmt::Display`.
impl fmt::Display for TriggerType {
    /// Formats the value for display.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The output formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The fmt result.
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
    /// Parses the value from its textual form.
    ///
    /// # Arguments
    ///
    /// - `&str` - The textual input.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The converted str, or the failure reason.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TRIGGER_PUSH => Ok(TriggerType::Push),
            TRIGGER_PULL_REQUEST => Ok(TriggerType::PullRequest),
            TRIGGER_MANUAL => Ok(TriggerType::Manual),
            TRIGGER_SCHEDULE => Ok(TriggerType::Schedule),
            TRIGGER_WEBHOOK => Ok(TriggerType::Webhook),
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
    /// - `bool` - True if the status is success, failure, cancelled, or skipped.
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
    /// - `bool` - True if the status is running.
    #[instrument_trace]
    pub fn is_active(self) -> bool {
        self == CicdStatus::Running
    }

    /// Checks whether this status represents a pending (waiting) state.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the status is pending.
    #[instrument_trace]
    pub fn is_pending(self) -> bool {
        self == CicdStatus::Pending
    }
}

/// Implementation of methods for `From`.
impl From<CicdStatus> for String {
    /// Builds the value from its component parts.
    ///
    /// # Arguments
    ///
    /// - `CicdStatus` - The target status.
    ///
    /// # Returns
    ///
    /// - `String` - The from result.
    fn from(status: CicdStatus) -> String {
        status.to_string()
    }
}
