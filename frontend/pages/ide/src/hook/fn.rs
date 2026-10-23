use super::*;

/// Creates the reactive state the ide page renders from.
///
/// The page calls no endpoint, so the state holds the language of the
/// scratchpad, the code it keeps and the message of the last failure.
///
/// # Returns
///
/// - `IdeState` - The state the page renders from.
pub fn use_ide_state() -> IdeState {
    let language: Signal<String> = App::use_signal(|| page_language().id);
    let language_name: Signal<String> = App::use_signal(|| page_language().name);
    let code: Signal<String> = App::use_signal(|| stored_code(&page_language().id));
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let state: IdeState = IdeState {
        language,
        language_name,
        code,
        error,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        install_error_listener(state);
        true
    });
    state
}

/// Keeps the code of the scratchpad.
///
/// The code is written into the storage of the browser, so the visitor finds
/// the code of the language again when it comes back to the page.
///
/// # Arguments
///
/// - `IdeState` - The state the page renders from.
/// - `String` - The code of the scratchpad.
///
/// # Returns
///
/// - `()` - The code is in the state and in the storage of the browser.
pub fn keep_code(state: IdeState, code: String) {
    let language: String = state.get_language().get();
    store_code(&language, &code);
    state.get_code().set(code);
}

/// Reports a failure of the page.
///
/// The failure is shown in the dialog of the page, so the visitor reads what
/// went wrong instead of losing it in the console of the browser.
///
/// # Arguments
///
/// - `IdeState` - The state the page renders from.
/// - `String` - The message of the failure.
///
/// # Returns
///
/// - `()` - The failure has been written into the state.
pub fn report_error(state: IdeState, message: String) {
    if message.trim().is_empty() {
        return;
    }
    state.get_error().set(message);
}

/// Installs the handler of the failures of the page.
///
/// The browser reports a failure of a script or of a promise through the two
/// events the page listens to.
///
/// # Arguments
///
/// - `IdeState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The handler of the failures is in the page.
fn install_error_listener(state: IdeState) {
    let Some(window): Option<Window> = window() else {
        return;
    };
    let handler: Closure<dyn FnMut(String)> = Closure::wrap(Box::new(move |message: String| {
        report_error(state, message);
    }));
    let _: Result<bool, JsValue> = js_sys::Reflect::set(
        &window,
        &JsValue::from_str(GLOBAL_ERROR),
        handler.as_ref(),
    );
    handler.forget();
    let script: String = format!(
        "window.addEventListener('{EVENT_ERROR}', function (event) {{ if (window.{GLOBAL_ERROR}) {{ window.{GLOBAL_ERROR}(event.message || ''); }} }}); window.addEventListener('{EVENT_REJECTION}', function (event) {{ if (window.{GLOBAL_ERROR}) {{ window.{GLOBAL_ERROR}(String(event.reason || '')); }} }});"
    );
    let _: Result<JsValue, JsValue> = js_sys::eval(&script);
}
