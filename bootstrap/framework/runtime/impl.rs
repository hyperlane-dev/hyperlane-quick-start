use super::*;

impl BootstrapSyncInit for RuntimeBootstrap {
    fn init() -> Self {
        let runtime: Runtime = Builder::new_multi_thread()
            .worker_threads(num_cpus::get_physical() << 1)
            .thread_stack_size(1_048_576)
            .max_blocking_threads(2_048)
            .max_io_events_per_tick(1_024)
            .enable_all()
            .build()
            .unwrap();
        Self { runtime }
    }
}
