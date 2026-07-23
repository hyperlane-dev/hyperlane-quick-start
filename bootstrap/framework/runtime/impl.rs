use super::*;

/// Implementation of `BootstrapSyncInit` for `RuntimeBootstrap`, creating a multi-threaded Tokio runtime on initialization.
impl BootstrapSyncInit for RuntimeBootstrap {
    /// Initializes the runtime bootstrap by creating a multi-threaded Tokio runtime with optimized worker and blocking thread counts.
    ///
    /// # Panics
    ///
    /// Panics if the Tokio runtime fails to build.
    ///
    /// # Returns
    ///
    /// - `Self`: The initialized `RuntimeBootstrap` instance containing the runtime.
    fn init() -> Self {
        let runtime: Runtime = Builder::new_multi_thread()
            .worker_threads(num_cpus::get_physical() << 1)
            .max_blocking_threads(2_048)
            .max_io_events_per_tick(1_024)
            .enable_all()
            .build()
            .unwrap();
        Self { runtime }
    }
}
