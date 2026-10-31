use super::*;

/// Renders the shortlink page.
///
/// The page holds the form that creates a short link and, below it, the
/// result card that shows the link the server answered with.
///
/// # Arguments
///
/// - `VirtualNode<ShortlinkPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn shortlink_page(node: VirtualNode<ShortlinkPageProps>) -> VirtualNode {
    let ShortlinkPageProps { state }: ShortlinkPageProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_PAGE
                title: TITLE_PAGE
                subtitle: SUBTITLE_PAGE
            }
            euv_card {
                title: TITLE_FORM
                div {
                    class: c_shortlink_form()
                    euv_field {
                        id: ID_URL
                        name: NAME_URL
                        label: LABEL_URL
                        input_type: TYPE_URL
                        placeholder: PLACEHOLDER_URL
                        value: state.get_url()
                        oninput: url_handler(state)
                    }
                    span {
                        class: c_shortlink_hint()
                        HINT_URL
                    }
                    div {
                        class: c_hls_button_row()
                        euv_button {
                            variant: VARIANT_PRIMARY
                            onclick: generate_handler(state)
                            LABEL_GENERATE
                        }
                    }
                }
            }
            if { state.get_loading().get() } {
                euv_loading {
                    title: TITLE_LOADING
                }
            }
            if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: VARIANT_ERROR
                    state.get_error().get().as_str()
                }
            }
            if { !state.get_shortlink().get().is_empty() } {
                euv_card {
                    title: TITLE_RESULT
                    div {
                        class: c_shortlink_result()
                        span {
                            class: c_shortlink_label()
                            SUBTITLE_RESULT
                        }
                        span {
                            class: c_shortlink_label()
                            LABEL_SHORTLINK
                        }
                        a {
                            class: c_shortlink_link()
                            href: state.get_shortlink().get()
                            target: "_blank"
                            state.get_shortlink().get().as_str()
                        }
                        div {
                            class: c_shortlink_actions()
                            div {
                                class: c_hls_button_row()
                                euv_button {
                                    variant: VARIANT_PRIMARY
                                    onclick: copy_handler(state)
                                    match { state.get_copied().get() } {
                                        true => {
                                            LABEL_COPIED
                                        }
                                        false => {
                                            LABEL_COPY
                                        }
                                    }
                                }
                            }
                            div {
                                class: c_hls_button_row()
                                euv_button {
                                    variant: VARIANT_OUTLINE
                                    onclick: open_handler(state)
                                    LABEL_OPEN
                                }
                            }
                            div {
                                class: c_hls_button_row()
                                euv_button {
                                    variant: VARIANT_OUTLINE
                                    onclick: reset_handler(state)
                                    LABEL_RESET
                                }
                            }
                        }
                    }
                }
            }
            euv_card {
                title: TITLE_INFO
                div {
                    class: c_shortlink_info()
                    span {
                        class: c_shortlink_info_item()
                        TEXT_INFO_ONE
                    }
                    span {
                        class: c_shortlink_info_item()
                        TEXT_INFO_TWO
                    }
                    span {
                        class: c_shortlink_info_item()
                        TEXT_INFO_THREE
                    }
                }
            }
        }
    }
}

/// Builds the handler that keeps the URL the visitor typed.
///
/// # Arguments
///
/// - `ShortlinkState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn url_handler(state: ShortlinkState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_url().set(input_value(&event));
        state.get_error().set(TEXT_EMPTY.to_string());
    }))
}

/// Builds the handler that asks the server for a short link.
///
/// # Arguments
///
/// - `ShortlinkState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn generate_handler(state: ShortlinkState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(create_shortlink(state));
    }))
}

/// Builds the handler that copies the short link to the clipboard.
///
/// # Arguments
///
/// - `ShortlinkState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn copy_handler(state: ShortlinkState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(copy_shortlink(state));
    }))
}

/// Builds the handler that opens the short link in a new tab.
///
/// # Arguments
///
/// - `ShortlinkState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_handler(state: ShortlinkState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let shortlink: String = state.get_shortlink().get();
        if shortlink.is_empty() {
            return;
        }
        if let Some(window) = window() {
            let _opened: Result<Option<Window>, JsValue> =
                window.open_with_url_and_target_and_features(&shortlink, "_blank", "");
        }
    }))
}

/// Builds the handler that clears the short link and the URL.
///
/// # Arguments
///
/// - `ShortlinkState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn reset_handler(state: ShortlinkState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        reset_shortlink(state);
    }))
}
