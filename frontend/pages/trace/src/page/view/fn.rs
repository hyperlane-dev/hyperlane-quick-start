use super::*;

/// Renders the trace page.
///
/// The page holds the field the visitor types the trace into and the answer
/// of the server, printed as the server wrote it.
///
/// # Arguments
///
/// - `VirtualNode<TracePageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn trace_page(node: VirtualNode<TracePageProps>) -> VirtualNode {
    let TracePageProps { state }: TracePageProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_TRACE
                title: TITLE_TRACE
                subtitle: SUBTITLE_TRACE
            }
            euv_card {
                title: TITLE_SEARCH
                div {
                    class: c_trace_search()
                    euv_input {
                        id: ID_TRACE
                        label: LABEL_TRACE
                        placeholder: PLACEHOLDER_TRACE
                        value: state.get_trace()
                        oninput: trace_handler(state)
                    }
                    div {
                        class: c_button_controls()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            onclick: search_handler(state)
                            LABEL_SEARCH
                        }
                    }
                }
            }
            euv_card {
                title: TITLE_RESULT
                if { !state.get_error().get().is_empty() } {
                    euv_alert {
                        variant: AlertVariant::Error
                        state.get_error().get().as_str()
                    }
                }
                match { state.get_loading().get() } {
                    true => {
                        euv_loading {
                            title: TITLE_LOADING
                        }
                    }
                    false => {
                        match { !state.get_searched().get() } {
                            true => {
                                euv_result {
                                    code: CODE_EMPTY
                                    title: TITLE_EMPTY
                                    description: TEXT_IDLE
                                }
                            }
                            false => {
                                pre {
                                    class: c_trace_output()
                                    state.get_result().get().as_str()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Builds the handler that keeps the trace the visitor is looking for.
///
/// # Arguments
///
/// - `TraceState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn trace_handler(state: TraceState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_trace().set(input_value(&event));
    }))
}

/// Builds the handler that asks the server for the trace.
///
/// # Arguments
///
/// - `TraceState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn search_handler(state: TraceState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(search_trace(state));
    }))
}
