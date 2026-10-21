use super::*;

/// Renders the redis page.
///
/// The page holds the form that creates a record, the list of the records
/// and the editor that changes the value of a record.
///
/// # Arguments
///
/// - `VirtualNode<RedisPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn redis_page(node: VirtualNode<RedisPageProps>) -> VirtualNode {
    let RedisPageProps { state }: RedisPageProps = node.try_get_props().unwrap_or_default();
    let records: Signal<Vec<RedisRecord>> = state.get_records();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_REDIS
                title: TITLE_REDIS
                subtitle: SUBTITLE_REDIS
            }
            if { !state.get_notice().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Success
                    state.get_notice().get().as_str()
                }
            } else if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Error
                    state.get_error().get().as_str()
                }
            }
            euv_card {
                title: TITLE_CREATE
                div {
                    class: c_redis_form()
                    euv_input {
                        id: ID_CREATE_KEY
                        label: LABEL_KEY
                        placeholder: PLACEHOLDER_KEY
                        value: state.get_create_key()
                        oninput: create_key_handler(state)
                    }
                    euv_input {
                        id: ID_CREATE_VALUE
                        label: LABEL_VALUE
                        placeholder: PLACEHOLDER_VALUE
                        value: state.get_create_value()
                        oninput: create_value_handler(state)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: create_handler(state)
                        LABEL_CREATE
                    }
                }
            }
            euv_card {
                title: TITLE_RECORDS
                div {
                    class: c_redis_toolbar()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: refresh_handler(state)
                        LABEL_REFRESH
                    }
                }
                if { state.get_loading().get() } {
                    euv_loading {
                        title: TITLE_LOADING
                    }
                } else if { records.get().is_empty() } {
                    euv_result {
                        code: CODE_EMPTY
                        title: TITLE_EMPTY
                        description: TEXT_NO_RECORD
                    }
                } else {
                    div {
                        class: c_redis_records()
                        for record in { records } {
                            record_row {
                                state: state
                                record: record
                            }
                        }
                    }
                }
            }
            if { state.get_edit_open().get() } {
                euv_modal {
                    title: TITLE_EDIT
                    onclick: close_handler(state)
                    div {
                        class: c_redis_form()
                        div {
                            class: c_redis_key()
                            state.get_edit_key().get().as_str()
                        }
                        euv_input {
                            id: ID_EDIT_VALUE
                            label: LABEL_VALUE
                            placeholder: PLACEHOLDER_VALUE
                            value: state.get_edit_value()
                            oninput: edit_value_handler(state)
                        }
                        div {
                            class: c_button_controls()
                            euv_button {
                                variant: EuvButtonVariant::Outline
                                onclick: close_handler(state)
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
        }
    }
}

/// Renders one record of the list.
///
/// # Arguments
///
/// - `VirtualNode<RecordProps>` - The props node carrying the record.
///
/// # Returns
///
/// - `VirtualNode` - The record tree.
#[component]
pub fn record_row(node: VirtualNode<RecordProps>) -> VirtualNode {
    let RecordProps { state, record }: RecordProps = node.try_get_props().unwrap_or_default();
    let key: String = record.key.clone();
    let value: String = record.value.clone();
    html! {
        div {
            key: record.key.clone()
            class: c_redis_record()
            div {
                class: c_redis_record_info()
                span {
                    class: c_redis_record_key()
                    key.as_str()
                }
                span {
                    class: c_redis_record_value()
                    value.as_str()
                }
            }
            div {
                class: c_redis_record_actions()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: edit_handler(state, key.clone(), value.clone())
                    LABEL_EDIT
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: delete_handler(state, key.clone())
                    LABEL_DELETE
                }
            }
        }
    }
}

/// Builds the handler that keeps the key of the record being created.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn create_key_handler(state: RedisState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_create_key().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the value of the record being created.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn create_value_handler(state: RedisState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_create_value().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the value of the record being edited.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn edit_value_handler(state: RedisState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_edit_value().set(input_value(&event));
    }))
}

/// Builds the handler that creates the record of the form.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn create_handler(state: RedisState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(create_record(state));
    }))
}

/// Builds the handler that saves the record of the editor.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn save_handler(state: RedisState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(update_record(state));
    }))
}

/// Builds the handler that closes the editor of a record.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_handler(state: RedisState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        close_editor(state);
    }))
}

/// Builds the handler that opens the editor of a record.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
/// - `String` - The key of the record to edit.
/// - `String` - The value of the record to edit.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn edit_handler(state: RedisState, key: String, value: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        open_editor(state, key.clone(), value.clone());
    }))
}

/// Builds the handler that removes a record.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
/// - `String` - The key of the record to remove.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn delete_handler(state: RedisState, key: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(delete_record(state, key.clone()));
    }))
}

/// Builds the handler that reloads the records of the cache.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn refresh_handler(state: RedisState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(load_records(state));
    }))
}
