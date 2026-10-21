use super::*;

/// Creates the reactive state the landing page renders from.
///
/// The page calls no endpoint, so the state holds the moment the page was
/// opened and the links of the card.
///
/// # Returns
///
/// - `TemplatesState` - The state the page renders from.
pub fn use_templates_state() -> TemplatesState {
    let time: Signal<String> = App::use_signal(current_time);
    let links: Signal<Vec<TemplatesLink>> = App::use_signal(|| {
        vec![
            TemplatesLink {
                label: LABEL_REPOSITORY.to_string(),
                url: URL_REPOSITORY.to_string(),
                target: TARGET_BLANK.to_string(),
            },
            TemplatesLink {
                label: LABEL_HOME.to_string(),
                url: URL_HOME.to_string(),
                target: String::new(),
            },
        ]
    });
    TemplatesState { time, links }
}

/// Returns the moment the browser is at, as a readable text.
///
/// # Returns
///
/// - `String` - The moment of the browser.
fn current_time() -> String {
    let now: i64 = js_sys::Date::now() as i64;
    let seconds: i64 = now / 1000;
    let minutes: i64 = seconds / 60;
    let hours: i64 = minutes / 60;
    let days: i64 = hours / 24;
    format!(
        "{days}d {hours}h {minutes}m {}s",
        seconds % 60
    )
}
