use super::*;

/// Reads the value of the text input the event came from.
///
/// # Arguments
///
/// - `&Event` - The event the value is read from.
///
/// # Returns
///
/// - `String` - The value of the input, empty when the event carried none.
pub fn input_value(event: &Event) -> String {
    let Some(target): Option<EventTarget> = event.target() else {
        return String::new();
    };
    let Ok(element): Result<HtmlInputElement, EventTarget> = target.dyn_into() else {
        return String::new();
    };
    element.value()
}

/// Reads the value of the text area the event came from.
///
/// # Arguments
///
/// - `&Event` - The event the value is read from.
///
/// # Returns
///
/// - `String` - The value of the text area, empty when the event carried none.
pub fn textarea_value(event: &Event) -> String {
    let Some(target): Option<EventTarget> = event.target() else {
        return String::new();
    };
    let Ok(element): Result<HtmlTextAreaElement, EventTarget> = target.dyn_into() else {
        return String::new();
    };
    element.value()
}
