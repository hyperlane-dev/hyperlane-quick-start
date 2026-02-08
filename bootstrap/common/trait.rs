pub trait BootstrapSyncInit {
    fn init() -> Self;
}

pub trait BootstrapAsyncInit {
    fn init() -> impl Future<Output = Self> + Send;
}
