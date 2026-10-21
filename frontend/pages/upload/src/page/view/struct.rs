use super::*;

/// The props of the upload page.
#[derive(Clone, Copy, Debug, Default)]
pub struct UploadPageProps {
    /// The state the page renders from.
    pub state: UploadState,
}

/// The props of one row of the upload list.
#[derive(Clone, Debug, Default)]
pub struct UploadRowProps {
    /// The state the page renders from.
    pub state: UploadState,
    /// The file the row shows.
    pub entry: UploadEntry,
}

/// The props of the progress bar of one row of the upload list.
#[derive(Clone, Copy, Debug, Default)]
pub struct UploadProgressProps {
    /// The share of the file that has been uploaded.
    pub percent: f64,
}

/// The props of one row of the upload history.
#[derive(Clone, Debug, Default)]
pub struct UploadHistoryProps {
    /// The state the page renders from.
    pub state: UploadState,
    /// The record the row shows.
    pub item: UploadHistoryItem,
}
