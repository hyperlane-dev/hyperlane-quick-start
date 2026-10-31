use super::*;

/// Creates the reactive state the trace page renders from.
///
/// The page holds the answer of the server and the identifier of the trace
/// the visitor is looking for.
///
/// # Returns
///
/// - `TraceState` - The state the page renders from.
pub fn use_trace_state() -> TraceState {
    let trace: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let result: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let loading: Signal<bool> = App::use_signal(|| false);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let searched: Signal<bool> = App::use_signal(|| false);
    let state: TraceState = TraceState {
        trace,
        result,
        loading,
        error,
        searched,
    };
    state
}

/// Asks the server for the trace the visitor is looking for.
///
/// # Arguments
///
/// - `TraceState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The answer of the server has been written into the state.
pub async fn search_trace(state: TraceState) {
    let search: TraceSearch = TraceSearch {
        trace: state.get_trace().get().trim().to_string(),
    };
    if search.trace.is_empty() {
        state.get_result().set(TEXT_EMPTY.to_string());
        state.get_error().set(MESSAGE_TRACE_REQUIRED.to_string());
        state.get_searched().set(false);
        return;
    }
    state.get_loading().set(true);
    let path: String = trace_path(&search.trace);
    match request_text(HttpMethod::Get, &path, None).await {
        Ok(text) => {
            state.get_result().set(text);
            state.get_error().set(TEXT_EMPTY.to_string());
            state.get_searched().set(true);
        }
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}
