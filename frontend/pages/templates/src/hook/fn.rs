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

/// Returns the time of day the browser is at, as a readable text.
///
/// The browser counts the seconds of the epoch, so the seconds of the day are
/// the rest of the division, and the time is the one of the UTC day.
///
/// # Returns
///
/// - `String` - The time of the day.
fn current_time() -> String {
    let now: i64 = js_sys::Date::now() as i64;
    let seconds_of_day: i64 = (now / 1000) % SECONDS_PER_DAY;
    let hours: i64 = seconds_of_day / SECONDS_PER_HOUR;
    let minutes: i64 = (seconds_of_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let seconds: i64 = seconds_of_day % SECONDS_PER_MINUTE;
    format!("{hours:02}h {minutes:02}m {seconds:02}s")
}
