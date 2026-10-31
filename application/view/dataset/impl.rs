use super::*;

/// Implementation of `DatasetViewRoute` for `ServerHook`.
impl ServerHook for DatasetViewRoute {
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The .
    /// - `&mut Context` - The .
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles one request and writes the response.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream.
    /// - `&mut Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `Status` - The handle result.
    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => DATASET_VIEW_REDIRECT_PATH)
    )]
    /// Implements the `handle` operation on [`DatasetViewRoute`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Mutable reference to a `Stream` (mutated in place).
    /// - `&mut Context` - Mutable reference to a `Context` (mutated in place).
    ///
    /// # Returns
    ///
    /// - `Status` - A `Status` value.
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}
