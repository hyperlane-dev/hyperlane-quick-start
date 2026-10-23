use super::*;

/// Renders the JSON editor page.
///
/// The page holds the field the document is written in, the tree of the
/// document below it and the editor of one node above them.
///
/// # Arguments
///
/// - `VirtualNode<JsonPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn json_page(node: VirtualNode<JsonPageProps>) -> VirtualNode {
    let JsonPageProps { state }: JsonPageProps = node.try_get_props().unwrap_or_default();
    let nodes: Signal<Vec<JsonNode>> = state.get_nodes();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_JSON
                title: TITLE_HEADER
                subtitle: TEXT_HEADER
            }
            euv_card {
                title: TITLE_INPUT
                div {
                    class: c_json_form()
                    div {
                        class: c_form_label()
                        LABEL_DOCUMENT
                    }
                    textarea {
                        class: c_textarea_input()
                        placeholder: PLACEHOLDER_DOCUMENT
                        rows: ROWS_INPUT
                        value: state.get_input().get()
                        oninput: input_handler(state)
                    }
                    div {
                        class: c_hls_button_row()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            onclick: parse_handler(state)
                            LABEL_PARSE
                        }
                        euv_button {
                            variant: EuvButtonVariant::Outline
                            onclick: copy_handler(state)
                            LABEL_COPY
                        }
                    }
                }
            }
            match { !state.get_notice().get().is_empty() } {
                true => {
                    euv_alert {
                        variant: AlertVariant::Success
                        state.get_notice().get().as_str()
                    }
                }
                false => {
                    match { !state.get_error().get().is_empty() } {
                        true => {
                            euv_alert {
                                variant: AlertVariant::Error
                                state.get_error().get().as_str()
                            }
                        }
                        false => {
                            ""
                        }
                    }
                }
            }
            euv_card {
                title: TITLE_TREE
                span {
                    class: c_json_count()
                    top_level_text(&state).as_str()
                }
                match { state.get_nodes().get().is_empty() } {
                    true => {
                        euv_result {
                            code: "0"
                            title: TITLE_TREE
                            description: TEXT_EMPTY_TREE
                        }
                    }
                    _ => {
                        div {
                            class: c_json_tree()
                            for node in { nodes } {
                                json_row {
                                    state: state
                                    node: node
                                }
                            }
                        }
                    }
                }
            }
            if { !state.get_edit_path().get().is_empty() } {
                euv_modal {
                    title: TITLE_EDIT
                    onclick: close_editor_handler(state)
                    json_editor {
                        state: state
                    }
                }
            }
        }
    }
}

/// Renders one row of the tree of the document.
///
/// # Arguments
///
/// - `VirtualNode<JsonRowProps>` - The props node carrying the node.
///
/// # Returns
///
/// - `VirtualNode` - The row tree.
#[component]
pub fn json_row(node: VirtualNode<JsonRowProps>) -> VirtualNode {
    let JsonRowProps { state, node }: JsonRowProps = node.try_get_props().unwrap_or_default();
    let path: Vec<String> = node.path.clone();
    let edit_path: Vec<String> = node.path.clone();
    let key: String = node.key.clone();
    let kind: String = node.kind.clone();
    let preview: String = node.preview.clone();
    let depth: usize = node.depth;
    let indent: String = format!("padding-left: calc({} * {depth})", var!(space-md));
    html! {
        div {
            key: format!("{path:?}")
            class: c_json_row()
            style: indent
            if node.container == true {
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: toggle_handler(state, path.clone())
                        if node.expanded == true {
                            GLYPH_OPEN
                        } else {
                            GLYPH_CLOSED
                        }
                    }
                }
            }
            span {
                class: c_json_key()
                key.as_str()
            }
            span {
                class: c_json_separator()
                ":"
            }
            span {
                class: c_json_type()
                kind.as_str()
            }
            span {
                class: c_json_value()
                preview.as_str()
            }
            if depth > 0 {
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: edit_handler(state, edit_path.clone())
                        LABEL_EDIT
                    }
                }
            }
        }
    }
}

/// Renders the editor of the node that is edited.
///
/// # Arguments
///
/// - `VirtualNode<JsonEditorProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The editor tree.
#[component]
pub fn json_editor(node: VirtualNode<JsonEditorProps>) -> VirtualNode {
    let JsonEditorProps { state }: JsonEditorProps = node.try_get_props().unwrap_or_default();
    let key: String = state.get_edit_key().get();
    let kind: String = state.get_edit_type().get();
    html! {
        div {
            class: c_json_form()
            div {
                class: c_form_label()
                LABEL_KEY
            }
            input {
                class: c_euv_input()
                value: key.as_str()
                readonly: true
            }
            div {
                class: c_form_label()
                LABEL_TYPE
            }
            input {
                class: c_euv_input()
                value: kind.as_str()
                readonly: true
            }
            div {
                class: c_form_label()
                LABEL_VALUE
            }
            textarea {
                class: c_textarea_input()
                rows: ROWS_VALUE
                value: state.get_edit_value().get()
                oninput: edit_value_handler(state)
            }
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: close_editor_handler(state)
                    LABEL_CANCEL
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    onclick: save_handler(state)
                    LABEL_SAVE
                }
            }
        }
    }
}

/// Builds the text of the number of the top level fields.
///
/// # Arguments
///
/// - `&JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `String` - The text of the number.
fn top_level_text(state: &JsonState) -> String {
    let count: usize = top_level_count(state);
    format!("{count} {TEXT_TOP_LEVEL}")
}

/// Builds the handler that keeps the text of the document.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn input_handler(state: JsonState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_input().set(textarea_value(&event));
    }))
}

/// Builds the handler that parses the document.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn parse_handler(state: JsonState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        parse_document(state);
    }))
}

/// Builds the handler that copies the document.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn copy_handler(state: JsonState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(copy_document(state));
    }))
}

/// Builds the handler that opens or closes one node of the tree.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
/// - `Vec<String>` - The path of the node.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn toggle_handler(state: JsonState, path: Vec<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        toggle_node(state, path.clone());
    }))
}

/// Builds the handler that opens the editor of one node of the tree.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
/// - `Vec<String>` - The path of the node.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn edit_handler(state: JsonState, path: Vec<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        open_editor(state, path.clone());
    }))
}

/// Builds the handler that keeps the value of the node that is edited.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn edit_value_handler(state: JsonState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_edit_value().set(textarea_value(&event));
    }))
}

/// Builds the handler that saves the value of the node that is edited.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn save_handler(state: JsonState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        save_edit(state);
    }))
}

/// Builds the handler that closes the editor of the node that is edited.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_editor_handler(state: JsonState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_edit_path().set(Vec::new());
    }))
}
