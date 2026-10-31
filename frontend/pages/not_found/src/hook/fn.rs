use super::*;

/// Creates the reactive state the not found page renders from.
///
/// The page calls no endpoint, so the state only holds the status code.
///
/// # Returns
///
/// - `NotFoundState` - The state the page renders from.
pub fn use_not_found_state() -> NotFoundState {
    let code: Signal<String> = App::use_signal(|| CODE_NOT_FOUND.to_string());
    NotFoundState { code }
}
