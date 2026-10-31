use super::*;

/// Implementation of `UploadStatus` for `fmt::Display`.
impl fmt::Display for UploadStatus {
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
            UploadStatus::Pending => "Pending",
            UploadStatus::Uploading => "Uploading",
            UploadStatus::Done => "Done",
            UploadStatus::Failed => "Failed",
        };
        write!(formatter, "{label}")
    }
}
