use super::*;

/// Implementation of `CicdStatus` for `fmt::Display`.
impl fmt::Display for CicdStatus {
    /// Formats the status as the label the page shows.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The output formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The fmt result.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label: &str = match self {
            CicdStatus::Pending => "pending",
            CicdStatus::Running => "running",
            CicdStatus::Success => "success",
            CicdStatus::Failure => "failure",
            CicdStatus::Cancelled => "cancelled",
            CicdStatus::Skipped => "skipped",
        };
        write!(formatter, "{label}")
    }
}
