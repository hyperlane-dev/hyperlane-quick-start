use super::*;

/// Renders the diff page.
///
/// The page holds the two texts, the counters of the comparison and the
/// result, which is rendered either in two panes or in one.
///
/// # Arguments
///
/// - `VirtualNode<DiffPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn diff_page(node: VirtualNode<DiffPageProps>) -> VirtualNode {
    let DiffPageProps { state }: DiffPageProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🆚"
                title: "Smart Text Diff Tool"
                subtitle: "Compare two texts and see the lines and the words that changed."
            }
            div {
                class: c_diff_panes()
                euv_card {
                    title: TITLE_ORIGINAL
                    div {
                        class: c_form_label()
                        LABEL_ORIGINAL
                    }
                    textarea {
                        class: c_textarea_input()
                        id: ID_ORIGINAL
                        placeholder: PLACEHOLDER_ORIGINAL
                        rows: ROWS_TEXT
                        oninput: original_handler(state)
                    }
                    div {
                        class: c_button_controls()
                        euv_button {
                            variant: EuvButtonVariant::Outline
                            onclick: copy_original_handler(state)
                            LABEL_COPY
                        }
                    }
                }
                euv_card {
                    title: TITLE_MODIFIED
                    div {
                        class: c_form_label()
                        LABEL_MODIFIED
                    }
                    textarea {
                        class: c_textarea_input()
                        id: ID_MODIFIED
                        placeholder: PLACEHOLDER_MODIFIED
                        rows: ROWS_TEXT
                        oninput: modified_handler(state)
                    }
                    div {
                        class: c_button_controls()
                        euv_button {
                            variant: EuvButtonVariant::Outline
                            onclick: copy_modified_handler(state)
                            LABEL_COPY
                        }
                    }
                }
            }
            div {
                class: c_diff_toolbar()
                div {
                    class: c_diff_stats()
                    diff_stat {
                        label: LABEL_ADDED
                        value: state.get_added().get().to_string()
                    }
                    diff_stat {
                        label: LABEL_REMOVED
                        value: state.get_removed().get().to_string()
                    }
                    diff_stat {
                        label: LABEL_UNCHANGED
                        value: state.get_unchanged().get().to_string()
                    }
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: clear_handler(state)
                    LABEL_CLEAR
                }
            }
            euv_card {
                title: TITLE_RESULT
                euv_tabs {
                    items: VIEW_ITEMS.to_vec()
                    active: state.get_view()
                }
                if { state.get_original().get().is_empty() && state.get_modified().get().is_empty() } {
                    euv_result {
                        code: "0"
                        title: TITLE_EMPTY
                        description: TEXT_EMPTY_RESULT
                    }
                } else if { state.get_view().get() == VIEW_SPLIT } {
                    split_view {
                        state: state
                    }
                } else {
                    inline_view {
                        state: state
                    }
                }
            }
        }
    }
}

/// Renders the result of the comparison in two panes.
///
/// The left pane holds the original text, the right pane the modified text.
/// Both panes are rendered from the same rows, so the lines stay aligned.
///
/// # Arguments
///
/// - `VirtualNode<DiffPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The split view tree.
#[component]
pub fn split_view(node: VirtualNode<DiffPageProps>) -> VirtualNode {
    let DiffPageProps { state }: DiffPageProps = node.try_get_props().unwrap_or_default();
    let rows: Signal<Vec<DiffRow>> = state.get_rows();
    html! {
        div {
            class: c_diff_split()
            div {
                class: c_diff_panel()
                div {
                    class: c_diff_panel_header()
                    span {
                        class: c_diff_panel_mark()
                        MARK_REMOVED
                    }
                    span {
                        class: c_diff_panel_title()
                        LABEL_ORIGINAL
                    }
                }
                for (index, row) in { rows.get().into_iter().enumerate() } {
                    div {
                        key: format!("original-{index}")
                        class: c_diff_cell()
                        diff_cell {
                            row: row
                            side: DiffSide::Original
                        }
                    }
                }
            }
            div {
                class: c_diff_panel()
                div {
                    class: c_diff_panel_header()
                    span {
                        class: c_diff_panel_mark()
                        MARK_ADDED
                    }
                    span {
                        class: c_diff_panel_title()
                        LABEL_MODIFIED
                    }
                }
                for (index, row) in { rows.get().into_iter().enumerate() } {
                    div {
                        key: format!("modified-{index}")
                        class: c_diff_cell()
                        diff_cell {
                            row: row
                            side: DiffSide::Modified
                        }
                    }
                }
            }
        }
    }
}

/// Renders the line of one cell of the split view.
///
/// A cell holds the line of one side of the result; the cell of the other
/// side is rendered from the same row. The line is the root of the cell,
/// because the row that holds the cell carries the key of the comparison.
///
/// # Arguments
///
/// - `VirtualNode<DiffRowProps>` - The props node carrying the row and the side.
///
/// # Returns
///
/// - `VirtualNode` - The line tree.
#[component]
pub fn diff_cell(node: VirtualNode<DiffRowProps>) -> VirtualNode {
    let DiffRowProps { row, side }: DiffRowProps = node.try_get_props().unwrap_or_default();
    let kind: DiffRowKind = row.kind;
    let parts: Vec<DiffPart> = match side {
        DiffSide::Original => row.original,
        DiffSide::Modified => row.modified,
    };
    html! {
        div {
            class: diff_row_class(kind, side)
            diff_segments {
                parts: parts
            }
        }
    }
}

/// Renders the result of the comparison in one pane.
///
/// Every line of the result is rendered with a marker that tells whether the
/// line was kept, removed or added.
///
/// # Arguments
///
/// - `VirtualNode<DiffPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The inline view tree.
#[component]
pub fn inline_view(node: VirtualNode<DiffPageProps>) -> VirtualNode {
    let DiffPageProps { state }: DiffPageProps = node.try_get_props().unwrap_or_default();
    let rows: Signal<Vec<DiffRow>> = state.get_rows();
    html! {
        div {
            class: c_diff_inline()
            for row in { rows } {
                if row.kind == DiffRowKind::Unchanged {
                    div {
                        class: c_diff_inline_row()
                        div {
                            class: c_diff_inline_marker()
                            MARK_UNCHANGED
                        }
                        div {
                            class: c_diff_inline_content()
                            diff_segments {
                                parts: row.original.clone()
                            }
                        }
                    }
                } else if row.kind == DiffRowKind::Changed {
                    div {
                        class: c_diff_inline_row()
                        div {
                            class: c_diff_inline_marker()
                            MARK_REMOVED
                        }
                        div {
                            class: c_diff_inline_content()
                            diff_segments {
                                parts: row.original.clone()
                            }
                        }
                    }
                    div {
                        class: c_diff_inline_row()
                        div {
                            class: c_diff_inline_marker()
                            MARK_ADDED
                        }
                        div {
                            class: c_diff_inline_content()
                            diff_segments {
                                parts: row.modified.clone()
                            }
                        }
                    }
                } else if row.kind == DiffRowKind::Removed {
                    div {
                        class: c_diff_inline_row()
                        div {
                            class: c_diff_inline_marker()
                            MARK_REMOVED
                        }
                        div {
                            class: c_diff_inline_content()
                            diff_segments {
                                parts: row.original.clone()
                            }
                        }
                    }
                } else if row.kind == DiffRowKind::Added {
                    div {
                        class: c_diff_inline_row()
                        div {
                            class: c_diff_inline_marker()
                            MARK_ADDED
                        }
                        div {
                            class: c_diff_inline_content()
                            diff_segments {
                                parts: row.modified.clone()
                            }
                        }
                    }
                } else {
                    div {
                        class: c_diff_inline_row()
                        div {
                            class: c_diff_inline_marker()
                            MARK_UNCHANGED
                        }
                        div {
                            class: c_diff_inline_content()
                        }
                    }
                }
            }
        }
    }
}

/// Renders the segments of one line.
///
/// A segment that was removed is marked, and so is a segment that was added.
///
/// # Arguments
///
/// - `VirtualNode<DiffSegmentsProps>` - The props node carrying the segments.
///
/// # Returns
///
/// - `VirtualNode` - The segments tree.
#[component]
pub fn diff_segments(node: VirtualNode<DiffSegmentsProps>) -> VirtualNode {
    let DiffSegmentsProps { parts }: DiffSegmentsProps = node.try_get_props().unwrap_or_default();
    html! {
        for part in parts {
            span {
                class: diff_part_class(part.kind)
                part.value.as_str()
            }
        }
    }
}

/// Renders one metric of the result.
///
/// # Arguments
///
/// - `VirtualNode<DiffStatProps>` - The props node carrying the metric.
///
/// # Returns
///
/// - `VirtualNode` - The metric tree.
#[component]
pub fn diff_stat(node: VirtualNode<DiffStatProps>) -> VirtualNode {
    let DiffStatProps { label, value }: DiffStatProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_hls_stat_card()
            div {
                class: c_hls_stat_value()
                value
            }
            div {
                class: c_hls_stat_label()
                label
            }
        }
    }
}

/// Returns the class of a line of the result.
///
/// # Arguments
///
/// - `DiffRowKind` - The kind of the row.
/// - `DiffSide` - The side of the result the line belongs to.
///
/// # Returns
///
/// - `&'static Css` - The class of the line.
fn diff_row_class(kind: DiffRowKind, side: DiffSide) -> &'static Css {
    match kind {
        DiffRowKind::Unchanged => c_diff_line(),
        DiffRowKind::Changed => match side {
            DiffSide::Original => c_diff_line_removed(),
            DiffSide::Modified => c_diff_line_added(),
        },
        DiffRowKind::Removed => match side {
            DiffSide::Original => c_diff_line_removed(),
            DiffSide::Modified => c_diff_line_empty(),
        },
        DiffRowKind::Added => match side {
            DiffSide::Original => c_diff_line_empty(),
            DiffSide::Modified => c_diff_line_added(),
        },
    }
}

/// Returns the class of a segment of a line.
///
/// # Arguments
///
/// - `DiffKind` - The kind of the segment.
///
/// # Returns
///
/// - `&'static Css` - The class of the segment.
fn diff_part_class(kind: DiffKind) -> &'static Css {
    match kind {
        DiffKind::Unchanged => c_diff_segment(),
        DiffKind::Removed => c_diff_segment_removed(),
        DiffKind::Added => c_diff_segment_added(),
    }
}

/// Builds the handler that keeps the original text.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn original_handler(state: DiffState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_original().set(textarea_value(&event));
        recompute(state);
    }))
}

/// Builds the handler that keeps the modified text.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn modified_handler(state: DiffState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_modified().set(textarea_value(&event));
        recompute(state);
    }))
}

/// Builds the handler that copies the original text.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn copy_original_handler(state: DiffState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let text: String = state.get_original().get();
        copy_to_clipboard(&text);
    }))
}

/// Builds the handler that copies the modified text.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn copy_modified_handler(state: DiffState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let text: String = state.get_modified().get();
        copy_to_clipboard(&text);
    }))
}

/// Builds the handler that clears both texts.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn clear_handler(state: DiffState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_original().set(TEXT_EMPTY.to_string());
        state.get_modified().set(TEXT_EMPTY.to_string());
        recompute(state);
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
