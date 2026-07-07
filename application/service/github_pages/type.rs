use super::*;

/// Type alias for the value carried by the watch channel in [`PendingFetch`].
pub type FetchPendingValue = Option<Result<Vec<u8>, String>>;

/// Receiver half of a [`PendingFetch`] watch channel.
pub type FetchPendingReceiver = watch::Receiver<FetchPendingValue>;

/// Sender half of a [`PendingFetch`] watch channel.
pub type FetchPendingSender = watch::Sender<FetchPendingValue>;
