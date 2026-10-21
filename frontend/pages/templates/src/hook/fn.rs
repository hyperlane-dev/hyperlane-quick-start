use super::*;

/// Creates the reactive state the landing page renders from.
///
/// The page calls no endpoint, so the state holds the moment the page was
/// served.
///
/// # Returns
///
/// - `TemplatesState` - The state the page renders from.
pub fn use_templates_state() -> TemplatesState {
    let time: Signal<String> = App::use_signal(current_time);
    TemplatesState { time }
}

/// Returns the moment the browser is at, as a readable text.
///
/// # Returns
///
/// - `String` - The moment of the browser.
fn current_time() -> String {
    let now: i64 = js_sys::Date::now() as i64;
    let seconds: i64 = now / 1000;
    let minutes: i64 = seconds / SECONDS_PER_MINUTE;
    let hours: i64 = minutes / MINUTES_PER_HOUR;
    let days: i64 = hours / HOURS_PER_DAY;
    format!(
        "{days}d {hours}h {minutes}m {}s",
        seconds % SECONDS_PER_MINUTE
    )
}
