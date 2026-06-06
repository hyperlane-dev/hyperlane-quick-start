/// Trait for synchronous bootstrap initialization, returning the initialized instance.
pub trait BootstrapSyncInit {
    /// Initializes the component synchronously and returns the instance.
    fn init() -> Self;
}

/// Trait for asynchronous bootstrap initialization, returning the initialized instance.
pub trait BootstrapAsyncInit {
    /// Initializes the component asynchronously and returns the instance.
    fn init() -> impl Future<Output = Self> + Send;
}
