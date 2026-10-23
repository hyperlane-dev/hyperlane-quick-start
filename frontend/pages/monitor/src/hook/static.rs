use super::*;

thread_local! {
    /// The live status stream of the page.
    pub static STREAM: RefCell<Option<MonitorStream>> = const { RefCell::new(None) };
}
