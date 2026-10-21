use super::*;

/// Renders the QR code page.
///
/// The page holds the list of the codes; every entry carries the text that
/// is encoded and the canvas the code is drawn into.
///
/// # Arguments
///
/// - `VirtualNode<QrcodePageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn qrcode_page(node: VirtualNode<QrcodePageProps>) -> VirtualNode {
    let QrcodePageProps { state }: QrcodePageProps = node.try_get_props().unwrap_or_default();
    let entries: Signal<Vec<QrEntry>> = state.get_entries();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_PAGE
                title: TITLE_PAGE
                subtitle: SUBTITLE_PAGE
            }
            if { !state.get_message().get().is_empty() } {
                euv_alert {
                    variant: if { state.get_failed().get() } {
                        VARIANT_ERROR
                    } else {
                        VARIANT_SUCCESS
                    }
                    state.get_message().get().as_str()
                }
            }
            euv_card {
                title: TITLE_LIST
                div {
                    class: c_qrcode_toolbar()
                    label {
                        class: c_qrcode_import()
                        input {
                            type: "file"
                            class: c_file_upload_input_hidden()
                            accept: ".json"
                            onchange: import_handler(state)
                        }
                        span {
                            class: c_qrcode_import_label()
                            LABEL_IMPORT
                        }
                    }
                    euv_button {
                        variant: VARIANT_PRIMARY
                        onclick: add_handler(state)
                        LABEL_ADD
                    }
                    euv_button {
                        variant: VARIANT_OUTLINE
                        onclick: export_handler(state)
                        LABEL_EXPORT
                    }
                    euv_button {
                        variant: VARIANT_OUTLINE
                        onclick: clear_handler(state)
                        LABEL_CLEAR
                    }
                    euv_dropdown {
                        open: state.get_level_open()
                        items: LEVEL_ITEMS.to_vec()
                        on_select: level_handler(state)
                        euv_button {
                            variant: VARIANT_OUTLINE
                            onclick: level_toggle_handler(state)
                            LABEL_LEVEL
                        }
                    }
                }
                if { entries.get().is_empty() } {
                    euv_result {
                        code: "0"
                        title: TITLE_EMPTY
                        description: TEXT_EMPTY_LIST
                    }
                } else {
                    div {
                        class: c_qrcode_list()
                        for (index, entry) in { entries.get().iter().enumerate() } {
                            qrcode_entry {
                                state: state
                                index: index
                                text: entry.text.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders one entry of the list of the codes.
///
/// # Arguments
///
/// - `VirtualNode<QrcodeEntryProps>` - The props node carrying the entry.
///
/// # Returns
///
/// - `VirtualNode` - The entry tree.
#[component]
pub fn qrcode_entry(node: VirtualNode<QrcodeEntryProps>) -> VirtualNode {
    let QrcodeEntryProps { state, index, text }: QrcodeEntryProps =
        node.try_get_props().unwrap_or_default();
    let canvas: NodeRef<HtmlCanvasElement> = App::use_node_ref();
    let level: QrErrorLevel = state.get_level().get();
    let initial_canvas: NodeRef<HtmlCanvasElement> = canvas.clone();
    let initial_text: String = text.clone();
    let initial_level: QrErrorLevel = level;
    let _drawn: Signal<bool> = App::use_signal(|| {
        schedule_draw(initial_canvas.clone(), initial_text.clone(), initial_level);
        true
    });
    let painted_canvas: NodeRef<HtmlCanvasElement> = canvas.clone();
    let painted_text: String = text.clone();
    draw_code(&painted_canvas, &painted_text, level);
    html! {
        div {
            class: c_qrcode_item()
            div {
                class: c_qrcode_item_header()
                span {
                    class: c_qrcode_item_title()
                    format!("[{}]", index + 1).as_str()
                }
                euv_button {
                    variant: VARIANT_OUTLINE
                    onclick: remove_handler(state, index)
                    LABEL_REMOVE
                }
            }
            textarea {
                class: c_textarea_input()
                placeholder: PLACEHOLDER_TEXT
                rows: ROWS_TEXT
                value: text.clone()
                oninput: text_handler(state, index, canvas.clone())
            }
            canvas {
                class: c_qrcode_canvas()
                ref: canvas
            }
            if { is_link(&text) } {
                euv_button {
                    variant: VARIANT_PRIMARY
                    onclick: open_handler(text.clone())
                    LABEL_OPEN
                }
            }
        }
    }
}

/// Builds the handler that writes the error correction level of the codes.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler.
fn level_handler(state: QrcodeState) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        set_level(state, value);
    }))
}

/// Builds the handler that opens the menu of the error correction levels.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn level_toggle_handler(state: QrcodeState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let open: bool = state.get_level_open().get();
        state.get_level_open().set(!open);
    }))
}

/// Builds the handler that keeps the text of one entry.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
/// - `usize` - The index of the entry.
/// - `NodeRef<HtmlCanvasElement>` - The canvas the code is drawn into.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn text_handler(
    state: QrcodeState,
    index: usize,
    canvas: NodeRef<HtmlCanvasElement>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let value: String = textarea_value(&event);
        set_entry_text(state, index, value.clone());
        let level: QrErrorLevel = state.get_level().get();
        draw_code(&canvas, &value, level);
    }))
}

/// Builds the handler that adds one entry to the list.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn add_handler(state: QrcodeState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        add_entry(state);
    }))
}

/// Builds the handler that removes one entry of the list.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
/// - `usize` - The index of the entry.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn remove_handler(state: QrcodeState, index: usize) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        remove_entry(state, index);
    }))
}

/// Builds the handler that clears the list of the codes.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn clear_handler(state: QrcodeState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        clear_entries(state);
    }))
}

/// Builds the handler that writes the list of the codes into a file.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn export_handler(state: QrcodeState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        export_entries(state);
    }))
}

/// Builds the handler that imports the list of the codes from a file.
///
/// # Arguments
///
/// - `QrcodeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The change handler.
fn import_handler(state: QrcodeState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(file): Option<File> = picked_file(&event) else {
            return;
        };
        spawn_local(read_and_import(state, file));
    }))
}

/// Builds the handler that opens the link of one entry.
///
/// # Arguments
///
/// - `String` - The text of the entry.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_handler(text: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let Some(window): Option<Window> = window() else {
            return;
        };
        let _opened: Result<Option<Window>, JsValue> =
            window.open_with_url_and_target_and_features(&text, "_blank", "");
    }))
}

/// Reports whether one text is a link the browser can open.
///
/// # Arguments
///
/// - `&str` - The text to check.
///
/// # Returns
///
/// - `bool` - Whether the text is a link.
fn is_link(text: &str) -> bool {
    text.starts_with("http://") || text.starts_with("https://")
}
