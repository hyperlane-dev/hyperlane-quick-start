use super::*;

/// Mounts the editor into a container of the page.
///
/// The editor is mounted after the page rendered the container, so the
/// element the editor is mounted into exists when the editor is created.
/// The loader of the editor is injected into the page when it is missing, and
/// the editor is created as soon as the loader is ready.
///
/// # Arguments
///
/// - `NodeRef<HtmlElement>` - The container of the editor.
/// - `String` - The language of the editor.
/// - `String` - The code the editor starts with.
/// - `Rc<dyn Fn(String)>` - The handler of the changes of the editor.
///
/// # Returns
///
/// - `()` - The editor is mounted, or the browser refused the editor.
pub fn mount_editor(
    container: NodeRef<HtmlElement>,
    language: String,
    value: String,
    on_change: Rc<dyn Fn(String)>,
) {
    let Some(window): Option<Window> = window() else {
        return;
    };
    let change: Closure<dyn FnMut(String)> = Closure::wrap(Box::new(move |code: String| {
        on_change(code);
    }));
    let _: Result<bool, JsValue> = js_sys::Reflect::set(
        &window,
        &JsValue::from_str(GLOBAL_CHANGE),
        change.as_ref(),
    );
    change.forget();
    let pending: JsValue = Closure::once_into_js(move || {
        create_editor();
    });
    let _: Result<bool, JsValue> = js_sys::Reflect::set(
        &window,
        &JsValue::from_str(GLOBAL_PENDING),
        &pending,
    );
    let container: NodeRef<HtmlElement> = container.clone();
    let deferred: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        prepare(container.clone(), language.clone(), value.clone());
    }));
    let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        deferred.as_ref().unchecked_ref(),
        DEFER_MILLIS,
    );
    deferred.forget();
}

/// Writes the code into the editor of the page.
///
/// The editor keeps the code of the project that is open, so the page hands
/// the code of the project to the editor when it opens one.
///
/// # Arguments
///
/// - `&str` - The code the editor shows.
///
/// # Returns
///
/// - `()` - The editor shows the code, or the editor is not mounted yet.
pub fn set_editor_value(value: &str) {
    let script: String = format!(
        "if (window.{GLOBAL_EDITOR}) {{ window.{GLOBAL_EDITOR}.setValue({}); }}",
        json_string(value)
    );
    let _: Result<JsValue, JsValue> = js_sys::eval(&script);
}

/// Prepares the editor of the page and injects its loader.
///
/// # Arguments
///
/// - `NodeRef<HtmlElement>` - The container of the editor.
/// - `String` - The language of the editor.
/// - `String` - The code the editor starts with.
///
/// # Returns
///
/// - `()` - The loader of the editor is on its way.
fn prepare(container: NodeRef<HtmlElement>, language: String, value: String) {
    let Some(window): Option<Window> = window() else {
        return;
    };
    let Some(element): Option<HtmlElement> = container.get_cloned() else {
        return;
    };
    let _: Result<bool, JsValue> = js_sys::Reflect::set(
        &window,
        &JsValue::from_str(GLOBAL_CONTAINER),
        &element,
    );
    let _: Result<bool, JsValue> = js_sys::Reflect::set(
        &window,
        &JsValue::from_str(GLOBAL_LANGUAGE),
        &JsValue::from_str(&language),
    );
    let _: Result<bool, JsValue> = js_sys::Reflect::set(
        &window,
        &JsValue::from_str(GLOBAL_VALUE),
        &JsValue::from_str(&value),
    );
    ensure_loader();
}

/// Injects the loader of the editor into the page.
///
/// The loader is only injected once, so the page does not load it twice.
///
/// # Returns
///
/// - `()` - The loader of the editor is in the page, or the page refused it.
fn ensure_loader() {
    let Some(window): Option<Window> = window() else {
        return;
    };
    let Some(document): Option<Document> = window.document() else {
        return;
    };
    if LOADER_ADDED.with(|added: &Cell<bool>| added.get()) {
        return;
    }
    LOADER_ADDED.with(|added: &Cell<bool>| added.set(true));
    let Ok(script): Result<Element, JsValue> = document.create_element("script") else {
        return;
    };
    let _: Result<(), JsValue> = script.set_attribute("src", LOADER_SRC);
    let _: Result<(), JsValue> = script.set_attribute("async", "false");
    let on_load: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        start_editor();
    }));
    let _: Result<(), JsValue> =
        script.add_event_listener_with_callback(EVENT_LOAD, on_load.as_ref().unchecked_ref());
    on_load.forget();
    let Some(body): Option<HtmlElement> = document.body() else {
        return;
    };
    let _: Result<Node, JsValue> = body.append_child(&script);
}

/// Asks the loader of the editor for the editor itself.
///
/// The loader of the Monaco editor is an AMD loader, so the editor is loaded
/// through the `require` function it installs on the page.
///
/// # Returns
///
/// - `()` - The editor is on its way, or the page refused the editor.
fn start_editor() {
    let script: String = format!(
        "require.config({{ paths: {{ vs: '{MONACO_PATH}' }} }}); require(['vs/editor/editor.main'], function () {{ window.{GLOBAL_READY} = true; var pending = window.{GLOBAL_PENDING}; if (pending) {{ pending(); }} }});"
    );
    let _: Result<JsValue, JsValue> = js_sys::eval(&script);
}

/// Creates the editor of the page.
///
/// The editor is created in the container of the page, with the language and
/// the code the page handed over. The theme of the editor is the monochrome
/// theme of the shared crate, so the code of the page carries no colour.
///
/// # Returns
///
/// - `()` - The editor is created, or the page refused the editor.
fn create_editor() {
    define_monaco_themes();
    let script: String = format!(
        "(function () {{ var container = window.{GLOBAL_CONTAINER}; if (!container) {{ return; }}; if (window.{GLOBAL_EDITOR}) {{ window.{GLOBAL_EDITOR}.dispose(); window.{GLOBAL_EDITOR} = null; }}; window.{GLOBAL_EDITOR} = monaco.editor.create(container, {{ value: window.{GLOBAL_VALUE}, language: window.{GLOBAL_LANGUAGE}, fontSize: {FONT_SIZE}, tabSize: {TAB_SIZE}, automaticLayout: true, minimap: {{ enabled: false }}, scrollBeyondLastLine: false, theme: window.__euv_monaco_theme ? window.__euv_monaco_theme() : 'euv-light' }}); window.{GLOBAL_EDITOR}.onDidChangeModelContent(function () {{ if (window.{GLOBAL_CHANGE}) {{ window.{GLOBAL_CHANGE}(window.{GLOBAL_EDITOR}.getValue()); }}; }}); }})();"
    );
    let _: Result<JsValue, JsValue> = js_sys::eval(&script);
    let follow: String = monaco_theme_script();
    let _: Result<JsValue, JsValue> = js_sys::eval(&follow);
}

/// Writes a text as a JavaScript string literal.
///
/// # Arguments
///
/// - `&str` - The text to write.
///
/// # Returns
///
/// - `String` - The text as a JavaScript string literal.
fn json_string(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_default()
}
