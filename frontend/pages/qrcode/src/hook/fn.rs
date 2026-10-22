use super::*;

/// Creates the reactive state the QR code page renders from.
///
/// The state holds the entries of the list of the codes; the list is read
/// out of the browser storage as soon as the page is mounted.
///
/// # Returns
///
/// - `QrcodeState` - The state the page renders from.
pub fn use_qrcode_state() -> QrcodeState {
    let entries: Signal<Vec<QrEntry>> = App::use_signal(Vec::new);
    let level: Signal<QrErrorLevel> = App::use_signal(|| QrErrorLevel::Medium);
    let level_open: Signal<bool> = App::use_signal(|| false);
    let message: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let failed: Signal<bool> = App::use_signal(|| false);
    let state: QrcodeState = QrcodeState {
        entries,
        level,
        level_open,
        message,
        failed,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        load_entries(state);
        true
    });
    state
}

/// Reads the list of the QR codes out of the browser storage.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
pub fn load_entries(state: QrcodeState) {
    let Some(storage): Option<Storage> = storage() else {
        state.get_entries().set(vec![QrEntry::default()]);
        return;
    };
    let Some(raw): Option<String> = storage.get_item(STORAGE_KEY).ok().flatten() else {
        state.get_entries().set(vec![QrEntry::default()]);
        return;
    };
    let Some(entries): Option<Vec<QrEntry>> = parse_entries(&raw) else {
        state.get_entries().set(vec![QrEntry::default()]);
        return;
    };
    let entries: Vec<QrEntry> = if entries.is_empty() {
        vec![QrEntry::default()]
    } else {
        entries
    };
    state.get_entries().set(entries);
}

/// Writes the list of the QR codes into the browser storage.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The list has been stored.
pub fn store_entries(state: QrcodeState) {
    let Some(storage): Option<Storage> = storage() else {
        return;
    };
    let Ok(encoded): Result<String, serde_json::Error> =
        serde_json::to_string(&state.get_entries().get())
    else {
        return;
    };
    let _stored: Result<(), JsValue> = storage.set_item(STORAGE_KEY, &encoded);
}

/// Writes the error correction level of the codes into the state.
///
/// The value is the one the menu of the level carries; an unknown value
/// leaves the level of the page alone.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
/// - `&str` - The value of the chosen entry of the menu.
///
/// # Returns
///
/// - `()` - The level has been written.
pub fn set_level(state: QrcodeState, value: &str) {
    let level: QrErrorLevel = match value {
        LEVEL_LOW => QrErrorLevel::Low,
        LEVEL_QUARTILE => QrErrorLevel::Quartile,
        LEVEL_HIGH => QrErrorLevel::High,
        _ => QrErrorLevel::Medium,
    };
    state.get_level().set(level);
}

/// Adds one empty entry to the list of the QR codes.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The entry has been added.
pub fn add_entry(state: QrcodeState) {
    let mut entries: Vec<QrEntry> = state.get_entries().get();
    entries.push(QrEntry::default());
    state.get_entries().set(entries);
    store_entries(state);
}

/// Removes one entry of the list of the QR codes.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
/// - `usize` - The index of the entry.
///
/// # Returns
///
/// - `()` - The entry has been removed.
pub fn remove_entry(state: QrcodeState, index: usize) {
    let mut entries: Vec<QrEntry> = state.get_entries().get();
    if index >= entries.len() {
        return;
    }
    entries.remove(index);
    if entries.is_empty() {
        entries.push(QrEntry::default());
    }
    state.get_entries().set(entries);
    store_entries(state);
}

/// Removes every entry of the list of the QR codes.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The list has been cleared.
pub fn clear_entries(state: QrcodeState) {
    state.get_entries().set(vec![QrEntry::default()]);
    store_entries(state);
    state.get_message().set(TEXT_EMPTY.to_string());
    state.get_failed().set(false);
}

/// Writes the text of one entry into the list of the QR codes.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
/// - `usize` - The index of the entry.
/// - `String` - The text the QR code encodes.
///
/// # Returns
///
/// - `()` - The entry has been written.
pub fn set_entry_text(state: QrcodeState, index: usize, text: String) {
    let mut entries: Vec<QrEntry> = state.get_entries().get();
    if index >= entries.len() {
        return;
    }
    entries[index].text = text;
    entries[index].error = TEXT_EMPTY.to_string();
    state.get_entries().set(entries);
    store_entries(state);
}

/// Writes the list of the QR codes into a file the browser downloads.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The file has been downloaded.
pub fn export_entries(state: QrcodeState) {
    let export: QrExport = QrExport {
        url_list: state.get_entries().get(),
    };
    let Ok(encoded): Result<String, serde_json::Error> = serde_json::to_string(&export) else {
        return;
    };
    download_file(EXPORT_FILE_NAME, &encoded);
    state
        .get_message()
        .set("The list of the QR codes has been exported".to_string());
    state.get_failed().set(false);
}

/// Reads the list of the QR codes out of an imported file.
///
/// The file holds either the list of the entries, the list under the key
/// `url_list` or one single entry.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
/// - `&str` - The content of the imported file.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
pub fn import_entries(state: QrcodeState, content: &str) {
    let Some(entries): Option<Vec<QrEntry>> = parse_entries(content) else {
        state
            .get_message()
            .set("The imported file does not hold a list of QR codes".to_string());
        state.get_failed().set(true);
        return;
    };
    let entries: Vec<QrEntry> = if entries.is_empty() {
        vec![QrEntry::default()]
    } else {
        entries
    };
    state.get_entries().set(entries);
    store_entries(state);
    state
        .get_message()
        .set("The list of the QR codes has been imported".to_string());
    state.get_failed().set(false);
}

/// Draws the QR code of one text into a canvas.
///
/// # Arguments
///
/// - `&NodeRef<HtmlCanvasElement>` - The canvas the code is drawn into.
/// - `&str` - The text the code encodes.
/// - `QrErrorLevel` - The error correction level of the code.
///
/// # Returns
///
/// - `bool` - Whether the code has been drawn.
pub fn draw_code(canvas: &NodeRef<HtmlCanvasElement>, text: &str, level: QrErrorLevel) -> bool {
    if text.trim().is_empty() {
        clear_code(canvas);
        return false;
    }
    let Some(code): Option<QrCode> = encode(text, level) else {
        return false;
    };
    let Some(element): Option<HtmlCanvasElement> = canvas.get_cloned() else {
        return false;
    };
    let modules: u32 = (code.size + QUIET_ZONE * 2) as u32;
    let width: u32 = modules * MODULE_PIXELS;
    element.set_width(width);
    element.set_height(width);
    let Some(context): Option<Object> = element.get_context("2d").ok().flatten() else {
        return false;
    };
    let Ok(context): Result<CanvasRenderingContext2d, Object> = context.dyn_into() else {
        return false;
    };
    context.set_fill_style_str(&theme_color(COLOR_LIGHT, FALLBACK_LIGHT));
    context.fill_rect(0.0, 0.0, width as f64, width as f64);
    context.set_fill_style_str(&theme_color(COLOR_DARK, FALLBACK_DARK));
    for row in 0..code.size {
        for column in 0..code.size {
            if !code.modules[row * code.size + column] {
                continue;
            }
            let left: f64 = ((column + QUIET_ZONE) * MODULE_PIXELS as usize) as f64;
            let top: f64 = ((row + QUIET_ZONE) * MODULE_PIXELS as usize) as f64;
            context.fill_rect(left, top, MODULE_PIXELS as f64, MODULE_PIXELS as f64);
        }
    }
    true
}

/// Clears the code of a canvas.
///
/// The canvas keeps the code it was given when the text of the entry is
/// emptied, so the code of the entry has to be wiped from the canvas.
///
/// # Arguments
///
/// - `&NodeRef<HtmlCanvasElement>` - The canvas the code was drawn into.
///
/// # Returns
///
/// - `()` - The canvas is empty.
fn clear_code(canvas: &NodeRef<HtmlCanvasElement>) {
    let Some(element): Option<HtmlCanvasElement> = canvas.get_cloned() else {
        return;
    };
    element.set_width(CANVAS_WIDTH);
    element.set_height(CANVAS_HEIGHT);
}

/// Schedules the drawing of the QR code of one text.
///
/// The drawing waits for the next tick of the browser, so the canvas is
/// already in the document when the code is drawn.
///
/// # Arguments
///
/// - `NodeRef<HtmlCanvasElement>` - The canvas the code is drawn into.
/// - `String` - The text the code encodes.
/// - `QrErrorLevel` - The error correction level of the code.
///
/// # Returns
///
/// - `()` - The drawing has been scheduled.
pub fn schedule_draw(canvas: NodeRef<HtmlCanvasElement>, text: String, level: QrErrorLevel) {
    let callback: JsValue = Closure::once_into_js(move || {
        draw_code(&canvas, &text, level);
    });
    let Some(window): Option<Window> = window() else {
        return;
    };
    let function: &js_sys::Function = callback.unchecked_ref::<js_sys::Function>();
    let _scheduled: Result<i32, JsValue> =
        window.set_timeout_with_callback_and_timeout_and_arguments_0(function, 0);
}

/// Reads the list of the QR codes out of the text of a file.
///
/// The file holds either the list of the entries, the list under the key
/// `url_list` or one single entry.
///
/// # Arguments
///
/// - `&str` - The content of the file.
///
/// # Returns
///
/// - `Option<Vec<QrEntry>>` - The list of the entries.
pub fn parse_entries(content: &str) -> Option<Vec<QrEntry>> {
    if let Ok(entries) = serde_json::from_str::<Vec<QrEntry>>(content) {
        return Some(entries);
    }
    if let Ok(export) = serde_json::from_str::<QrExport>(content) {
        return Some(export.url_list);
    }
    if let Ok(entry) = serde_json::from_str::<QrEntry>(content) {
        return Some(vec![entry]);
    }
    None
}

/// Reads the content of a file the visitor picked.
///
/// # Arguments
///
/// - `&Event` - The event the file is read from.
///
/// # Returns
///
/// - `Option<File>` - The file of the event.
pub fn picked_file(event: &Event) -> Option<File> {
    let target: EventTarget = event.target()?;
    let input: HtmlInputElement = target.dyn_into().ok()?;
    let files: FileList = input.files()?;
    files.get(0)
}

/// Reads the browser storage of the page.
///
/// # Returns
///
/// - `Option<Storage>` - The storage, absent when the browser denies it.
fn storage() -> Option<Storage> {
    window()
        .and_then(|window: Window| window.local_storage().ok())
        .flatten()
}

/// Reads one colour of the theme of the page.
///
/// # Arguments
///
/// - `&str` - The name of the colour.
/// - `&str` - The colour to use when the theme holds none.
///
/// # Returns
///
/// - `String` - The colour of the theme.
fn theme_color(name: &str, fallback: &str) -> String {
    let Some(window): Option<Window> = window() else {
        return fallback.to_string();
    };
    let Some(document): Option<Document> = window.document() else {
        return fallback.to_string();
    };
    let Some(element): Option<Element> = document.document_element() else {
        return fallback.to_string();
    };
    let Ok(style): Result<Option<CssStyleDeclaration>, JsValue> =
        window.get_computed_style(&element)
    else {
        return fallback.to_string();
    };
    let Some(style) = style else {
        return fallback.to_string();
    };
    let value: String = style.get_property_value(name).unwrap_or_default();
    if value.trim().is_empty() {
        return fallback.to_string();
    }
    value.trim().to_string()
}

/// Writes a text into a file the browser downloads.
///
/// # Arguments
///
/// - `&str` - The name of the file.
/// - `&str` - The content of the file.
///
/// # Returns
///
/// - `bool` - Whether the file has been downloaded.
fn download_file(name: &str, content: &str) -> bool {
    let parts: JsValue = JsValue::from_str(content);
    let Ok(blob): Result<Blob, JsValue> = Blob::new_with_str_sequence(&parts) else {
        return false;
    };
    let Some(href): Option<String> = object_url(&blob) else {
        return false;
    };
    let Some(window): Option<Window> = window() else {
        return false;
    };
    let Some(document): Option<Document> = window.document() else {
        return false;
    };
    let Ok(element): Result<Element, JsValue> = document.create_element("a") else {
        return false;
    };
    let _href: Result<(), JsValue> = element.set_attribute("href", &href);
    let _download: Result<(), JsValue> = element.set_attribute("download", name);
    let Ok(html): Result<HtmlElement, Element> = element.dyn_into() else {
        return false;
    };
    html.click();
    true
}

/// Builds the address of the object behind a blob.
///
/// # Arguments
///
/// - `&Blob` - The blob the address is built for.
///
/// # Returns
///
/// - `Option<String>` - The address of the blob.
fn object_url(blob: &Blob) -> Option<String> {
    let global: JsValue = js_sys::global().into();
    let constructor: JsValue = Reflect::get(&global, &JsValue::from_str("URL")).ok()?;
    let factory: JsValue =
        Reflect::get(&constructor, &JsValue::from_str("createObjectURL")).ok()?;
    let function: js_sys::Function = factory.dyn_into().ok()?;
    let address: JsValue = function.call1(&constructor, &blob.clone().into()).ok()?;
    address.as_string()
}

/// Reads the content of a file the visitor picked.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
/// - `File` - The file to read.
///
/// # Returns
///
/// - `()` - The list has been imported.
pub async fn read_and_import(state: QrcodeState, file: File) {
    let promise: Promise = file.text();
    let Ok(value): Result<JsValue, JsValue> = JsFuture::from(promise).await else {
        return;
    };
    let content: String = value.as_string().unwrap_or_default();
    import_entries(state, &content);
}
