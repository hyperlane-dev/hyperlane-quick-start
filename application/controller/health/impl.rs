use super::*;

/// Implementation of `HealthCheckRoute` for `ServerHook`.
impl ServerHook for HealthCheckRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `HealthCheckRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method)]
    #[instrument_trace]
    /// Handles the `HealthCheckRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}
