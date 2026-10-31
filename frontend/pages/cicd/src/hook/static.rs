use super::*;

thread_local! {
    /// The log stream of the page.
    pub(crate) static LOG_STREAM: RefCell<Option<LogStream>> = const { RefCell::new(None) };
}
