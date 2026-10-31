use super::*;

/// Implementation of `MonitorStream`.
impl MonitorStream {
    /// Closes the stream and keeps the handlers bound to it alive.
    ///
    /// The handlers are released to the runtime, because the browser still
    /// holds the functions the closed stream was built with.
    ///
    /// # Returns
    ///
    /// - `()` - The stream is closed.
    pub fn close(self) {
        let MonitorStream {
            source,
            message,
            open,
            error,
        } = self;
        source.close();
        message.forget();
        open.forget();
        error.forget();
    }
}
