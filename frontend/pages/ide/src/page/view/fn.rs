use super::*;

/// Renders the ide page.
///
/// The page holds the scratchpad: the language the visitor works in, the
/// editor that shows the code and the dialog that reports a failure.
///
/// # Arguments
///
/// - `VirtualNode<IdePageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn ide_page(node: VirtualNode<IdePageProps>) -> VirtualNode {
    let IdePageProps { state }: IdePageProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "⌨️"
                title: "Hyperlane Online IDE"
                subtitle: "A scratchpad for a piece of code, kept in the browser."
            }
            div {
                class: c_ide_toolbar()
                div {
                    class: c_ide_language()
                    span {
                        class: c_ide_language_label()
                        LABEL_LANGUAGE
                    }
                    span {
                        class: c_ide_language_value()
                        {
                            state.get_language_name().get()
                        }
                    }
                }
                div {
                    class: c_hls_button_row()
                    div {
                        class: c_hls_button_row()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            onclick: copy_handler(state)
                            LABEL_COPY
                        }
                    }
                    div {
                        class: c_hls_button_row()
                        euv_button {
                            variant: EuvButtonVariant::Outline
                            onclick: clear_handler(state)
                            LABEL_CLEAR
                        }
                    }
                }
            }
            ide_editor {
                state: state
            }
            if { !state.get_error().get().is_empty() } {
                euv_modal {
                    title: TITLE_ERROR
                    onclick: close_error_handler(state)
                    pre {
                        class: c_ide_error()
                        state.get_error().get().as_str()
                    }
                }
            }
        }
    }
}

/// Renders the editor of the scratchpad.
///
/// The editor of the page is the vendored Monaco editor, so it is mounted
/// into the container of the page once the container is in the document.
///
/// # Arguments
///
/// - `VirtualNode<IdeEditorProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The editor tree.
#[component]
pub fn ide_editor(node: VirtualNode<IdeEditorProps>) -> VirtualNode {
    let IdeEditorProps { state }: IdeEditorProps = node.try_get_props().unwrap_or_default();
    let container: NodeRef<HtmlElement> = App::use_node_ref();
    let _started: Signal<bool> = App::use_signal(|| {
        let language: String = state.get_language().get();
        let value: String = state.get_code().get();
        let on_change: Rc<dyn Fn(String)> = Rc::new(move |code: String| {
            keep_code(state, code);
        });
        mount_editor(container.clone(), language, value, on_change);
        true
    });
    html! {
        div {
            class: c_ide_editor()
            ref: container
        }
    }
}

/// Builds the handler that copies the code of the scratchpad.
///
/// # Arguments
///
/// - `IdeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn copy_handler(state: IdeState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let code: String = state.get_code().get();
        copy_to_clipboard(&code);
    }))
}

/// Builds the handler that clears the code of the scratchpad.
///
/// # Arguments
///
/// - `IdeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn clear_handler(state: IdeState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        keep_code(state, TEXT_EMPTY.to_string());
        set_editor_value(TEXT_EMPTY);
    }))
}

/// Builds the handler that closes the dialog of a failure.
///
/// # Arguments
///
/// - `IdeState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_error_handler(state: IdeState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_error().set(TEXT_EMPTY.to_string());
    }))
}

/// Copies a text into the clipboard of the browser.
///
/// # Arguments
///
/// - `&str` - The text to copy.
///
/// # Returns
///
/// - `()` - The text has been handed to the clipboard of the browser.
fn copy_to_clipboard(text: &str) {
    let encoded: String = serde_json::to_string(text).unwrap_or_default();
    let script: String = format!("navigator.clipboard.writeText({encoded})");
    let _: Result<JsValue, JsValue> = js_sys::eval(&script);
}
