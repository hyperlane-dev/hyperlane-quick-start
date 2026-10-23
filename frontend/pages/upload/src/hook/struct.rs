use super::*;

/// The reactive state the upload page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct UploadState {
    /// The files of the upload list.
    #[get(type(copy))]
    pub entries: Signal<Vec<UploadEntry>>,
    /// The records of the upload history.
    #[get(type(copy))]
    pub history: Signal<Vec<UploadHistoryItem>>,
    /// The message of the last call.
    #[get(type(copy))]
    pub message: Signal<String>,
    /// Whether the message of the last call reports a failure.
    #[get(type(copy))]
    pub failed: Signal<bool>,
    /// Whether an upload is in flight.
    #[get(type(copy))]
    pub uploading: Signal<bool>,
}
