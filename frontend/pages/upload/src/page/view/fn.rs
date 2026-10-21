use super::*;

/// Renders the upload page.
///
/// The page holds the picker that uploads files in chunks, the list of the
/// files that are being uploaded and the history of the files the browser
/// uploaded before.
///
/// # Arguments
///
/// - `VirtualNode<UploadPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn upload_page(node: VirtualNode<UploadPageProps>) -> VirtualNode {
    let UploadPageProps { state }: UploadPageProps = node.try_get_props().unwrap_or_default();
    let entries: Signal<Vec<UploadEntry>> = state.get_entries();
    let history: Signal<Vec<UploadHistoryItem>> = state.get_history();
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
                title: TITLE_UPLOAD
                div {
                    class: c_upload_form()
                    label {
                        class: c_upload_picker()
                        input {
                            type: "file"
                            multiple: true
                            class: c_file_upload_input_hidden()
                            onchange: pick_handler(state)
                        }
                        span {
                            class: c_upload_picker_label()
                            LABEL_PICK
                        }
                    }
                    span {
                        class: c_upload_hint()
                        HINT_PICK
                    }
                    if { entries.get().is_empty() } {
                        euv_result {
                            code: "0"
                            title: TITLE_EMPTY
                            description: TEXT_EMPTY_LIST
                        }
                    } else {
                        div {
                            class: c_upload_list()
                            for entry in { entries } {
                                upload_row {
                                    state: state
                                    entry: entry
                                }
                            }
                        }
                    }
                }
            }
            euv_card {
                title: TITLE_HISTORY
                div {
                    class: c_upload_form()
                    div {
                        class: c_upload_actions()
                        euv_button {
                            variant: VARIANT_PRIMARY
                            onclick: export_handler(state)
                            LABEL_EXPORT
                        }
                        label {
                            class: c_upload_import()
                            input {
                                type: "file"
                                class: c_file_upload_input_hidden()
                                accept: ".json"
                                onchange: import_handler(state)
                            }
                            span {
                                class: c_upload_import_label()
                                LABEL_IMPORT
                            }
                        }
                    }
                    span {
                        class: c_upload_hint()
                        HINT_IMPORT
                    }
                    if { history.get().is_empty() } {
                        euv_result {
                            code: "0"
                            title: TITLE_EMPTY_HISTORY
                            description: TEXT_EMPTY_HISTORY
                        }
                    } else {
                        div {
                            class: c_upload_list()
                            for item in { history } {
                                upload_history_row {
                                    state: state
                                    item: item
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders one file of the upload list.
///
/// # Arguments
///
/// - `VirtualNode<UploadRowProps>` - The props node carrying the file.
///
/// # Returns
///
/// - `VirtualNode` - The row tree.
#[component]
pub fn upload_row(node: VirtualNode<UploadRowProps>) -> VirtualNode {
    let UploadRowProps { state, entry }: UploadRowProps = node.try_get_props().unwrap_or_default();
    let _state: UploadState = state;
    html! {
        div {
            key: entry.id.clone()
            class: c_upload_item()
            div {
                class: c_upload_item_header()
                span {
                    class: c_upload_item_name()
                    entry.name.as_str()
                }
                span {
                    class: c_upload_item_size()
                    format_size(entry.size).as_str()
                }
                span {
                    class: c_upload_item_status()
                    entry.status.to_string()
                }
            }
            upload_progress {
                percent: entry.progress
            }
            if !entry.error.is_empty() {
                span {
                    class: c_upload_item_error()
                    entry.error.as_str()
                }
            }
            if !entry.url.is_empty() {
                a {
                    class: c_upload_item_url()
                    href: entry.url.clone()
                    target: "_blank"
                    entry.url.as_str()
                }
            }
        }
    }
}

/// Renders the progress bar of one file of the upload list.
///
/// # Arguments
///
/// - `VirtualNode<UploadProgressProps>` - The props node carrying the share.
///
/// # Returns
///
/// - `VirtualNode` - The progress bar tree.
#[component]
pub fn upload_progress(node: VirtualNode<UploadProgressProps>) -> VirtualNode {
    let UploadProgressProps { percent }: UploadProgressProps = node.try_get_props().unwrap_or_default();
    let width: String = format!("width: {percent}%;");
    html! {
        div {
            class: c_upload_progress()
            div {
                class: c_upload_progress_fill()
                style: width.as_str()
            }
        }
    }
}

/// Renders one record of the upload history.
///
/// # Arguments
///
/// - `VirtualNode<UploadHistoryProps>` - The props node carrying the record.
///
/// # Returns
///
/// - `VirtualNode` - The row tree.
#[component]
pub fn upload_history_row(node: VirtualNode<UploadHistoryProps>) -> VirtualNode {
    let UploadHistoryProps { state, item }: UploadHistoryProps = node.try_get_props().unwrap_or_default();
    let _state: UploadState = state;
    html! {
        div {
            key: item.id.clone()
            class: c_upload_item()
            div {
                class: c_upload_item_header()
                span {
                    class: c_upload_item_name()
                    item.name.as_str()
                }
                span {
                    class: c_upload_item_size()
                    format_size(item.size).as_str()
                }
                span {
                    class: c_upload_item_size()
                    item.upload_time.as_str()
                }
            }
            if !item.url.is_empty() {
                a {
                    class: c_upload_item_url()
                    href: item.url.clone()
                    target: "_blank"
                    item.url.as_str()
                }
            }
        }
    }
}

/// Formats the size of a file as a short human readable text.
///
/// # Arguments
///
/// - `f64` - The size of the file in bytes.
///
/// # Returns
///
/// - `String` - The size of the file.
fn format_size(bytes: f64) -> String {
    if bytes >= BYTES_PER_GIGABYTE {
        return format!("{:.1} {UNIT_GIGABYTE}", bytes / BYTES_PER_GIGABYTE);
    }
    if bytes >= BYTES_PER_MEGABYTE {
        return format!("{:.1} {UNIT_MEGABYTE}", bytes / BYTES_PER_MEGABYTE);
    }
    if bytes >= BYTES_PER_KILOBYTE {
        return format!("{:.1} {UNIT_KILOBYTE}", bytes / BYTES_PER_KILOBYTE);
    }
    format!("{bytes:.0} {UNIT_BYTE}")
}

/// Builds the handler that uploads the files the visitor picked.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The change handler.
fn pick_handler(state: UploadState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(files) = picked_files(&event) else {
            return;
        };
        spawn_local(upload_files(state, files));
    }))
}

/// Builds the handler that imports an upload history from a file.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The change handler.
fn import_handler(state: UploadState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(files) = picked_files(&event) else {
            return;
        };
        let Some(file) = files.into_iter().next() else {
            return;
        };
        spawn_local(read_and_import(state, file));
    }))
}

/// Reads the files the visitor picked from a file input event.
///
/// # Arguments
///
/// - `&Event` - The event the files are read from.
///
/// # Returns
///
/// - `Option<Vec<File>>` - The files of the event.
fn picked_files(event: &Event) -> Option<Vec<File>> {
    let target: EventTarget = event.target()?;
    let input: HtmlInputElement = target.dyn_into().ok()?;
    let list: FileList = input.files()?;
    let count: u32 = list.length();
    let mut files: Vec<File> = Vec::new();
    for index in 0..count {
        if let Some(file) = list.get(index) {
            files.push(file);
        }
    }
    Some(files)
}

/// Reads the content of a file and imports it as an upload history.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `File` - The file to read.
///
/// # Returns
///
/// - `()` - The history has been imported.
async fn read_and_import(state: UploadState, file: File) {
    let promise: Promise = file.text();
    let Ok(value): Result<JsValue, JsValue> = euv::wasm_bindgen_futures::JsFuture::from(promise).await
    else {
        return;
    };
    let content: String = value.as_string().unwrap_or_default();
    import_history(state, &content);
}

/// Builds the handler that writes the upload history into a file.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn export_handler(state: UploadState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        export_history(state);
    }))
}
