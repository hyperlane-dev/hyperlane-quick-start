use super::*;

/// Type alias for the value carried by the watch channel in [`PendingFetch`].
pub type FetchPendingValue = Option<Result<Vec<u8>, String>>;

/// Receiver half of a [`PendingFetch`] watch channel.
pub type FetchPendingReceiver = watch::Receiver<FetchPendingValue>;

/// Sender half of a [`PendingFetch`] watch channel.
pub type FetchPendingSender = watch::Sender<FetchPendingValue>;

/// Outcome of a single GitHub Pages resource fetch: `(normalized_path, linked_resources)`
/// on success, or the normalized path on failure.
pub type FetchResourceOutcome = (String, Vec<String>);

/// Error returned when a GitHub Pages resource fetch fails.
pub type FetchResourceError = String;

/// Result of a single GitHub Pages resource fetch.
pub type FetchResourceResult = Result<FetchResourceOutcome, FetchResourceError>;

/// Send-error type for the unbounded mpsc channel that delivers
/// [`FetchResourceResult`] to sync-task notifiers.
pub type FetchResourceSendError = tokio::sync::mpsc::error::SendError<FetchResourceResult>;
