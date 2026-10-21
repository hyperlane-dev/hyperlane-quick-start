use super::*;

/// Renders the chat page.
///
/// The page holds the messages of the room, the list of the visitors that
/// can be mentioned and the field the message is written in.
///
/// # Arguments
///
/// - `VirtualNode<ChatPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn chat_page(node: VirtualNode<ChatPageProps>) -> VirtualNode {
    let ChatPageProps { state }: ChatPageProps = node.try_get_props().unwrap_or_default();
    let messages: Signal<Vec<ChatMessage>> = state.get_messages();
    let candidates: Signal<Vec<OnlineUser>> = state.get_candidates();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_PAGE
                title: TITLE_PAGE
                subtitle: SUBTITLE_PAGE
            }
            if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: VARIANT_ERROR
                    state.get_error().get().as_str()
                }
            }
            euv_card {
                title: TITLE_ROOM
                div {
                    class: c_chat_status()
                    span {
                        class: c_chat_status_state()
                        if { state.get_connected().get() } {
                            LABEL_CONNECTED
                        } else {
                            LABEL_DISCONNECTED
                        }
                    }
                    span {
                        class: c_chat_status_count()
                        state.get_online_count().get().as_str()
                    }
                    span {
                        class: c_chat_status_count()
                        LABEL_ONLINE
                    }
                }
                if { messages.get().is_empty() } {
                    euv_result {
                        code: "0"
                        title: TITLE_EMPTY
                        description: TEXT_EMPTY_ROOM
                    }
                } else {
                    div {
                        class: c_chat_messages()
                        for message in { messages } {
                            chat_message_row {
                                state: state
                                message: message
                            }
                        }
                    }
                }
                if { state.get_uploading().get() } {
                    euv_loading {
                        title: TEXT_UPLOADING
                    }
                }
                if { !state.get_upload_message().get().is_empty() } {
                    span {
                        class: c_chat_upload_message()
                        state.get_upload_message().get().as_str()
                    }
                }
                if { state.get_mention_open().get() } {
                    div {
                        class: c_chat_mentions()
                        span {
                            class: c_chat_mentions_title()
                            TITLE_MENTION
                        }
                        if { candidates.get().is_empty() } {
                            span {
                                class: c_chat_mentions_empty()
                                TEXT_EMPTY_MENTION
                            }
                        } else {
                            for user in { candidates } {
                                mention_row {
                                    state: state
                                    user: user
                                }
                            }
                        }
                    }
                }
                div {
                    class: c_chat_composer()
                    textarea {
                        class: c_textarea_input()
                        placeholder: PLACEHOLDER_MESSAGE
                        rows: ROWS_MESSAGE
                        oninput: draft_handler(state)
                        onkeydown: key_handler(state)
                    }
                    div {
                        class: c_chat_actions()
                        euv_button {
                            variant: VARIANT_PRIMARY
                            onclick: send_handler(state)
                            LABEL_SEND
                        }
                        label {
                            class: c_chat_attach()
                            input {
                                type: "file"
                                class: c_file_upload_input_hidden()
                                onchange: attach_handler(state)
                            }
                            span {
                                class: c_chat_attach_label()
                                LABEL_ATTACH
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders one message of the room.
///
/// # Arguments
///
/// - `VirtualNode<ChatMessageProps>` - The props node carrying the message.
///
/// # Returns
///
/// - `VirtualNode` - The message tree.
#[component]
pub fn chat_message_row(node: VirtualNode<ChatMessageProps>) -> VirtualNode {
    let ChatMessageProps { state, message }: ChatMessageProps =
        node.try_get_props().unwrap_or_default();
    let _state: ChatState = state;
    let sender: String = if message.own {
        LABEL_YOU.to_string()
    } else {
        message.sender.clone()
    };
    let time: String = format_time(message.time);
    html! {
        div {
            class: c_chat_message()
            class: if message.own {
                c_chat_message_own()
            } else {
                c_chat_message_other()
            }
            div {
                class: c_chat_message_header()
                span {
                    class: c_chat_message_sender()
                    sender.as_str()
                }
                span {
                    class: c_chat_message_time()
                    time.as_str()
                }
            }
            p {
                class: c_chat_message_content()
                message.content.as_str()
            }
        }
    }
}

/// Renders one visitor of the mention list.
///
/// # Arguments
///
/// - `VirtualNode<MentionProps>` - The props node carrying the visitor.
///
/// # Returns
///
/// - `VirtualNode` - The visitor tree.
#[component]
pub fn mention_row(node: VirtualNode<MentionProps>) -> VirtualNode {
    let MentionProps { state, user }: MentionProps = node.try_get_props().unwrap_or_default();
    let name: String = user.username.clone();
    html! {
        euv_button {
            variant: VARIANT_OUTLINE
            onclick: mention_handler(state, name.clone())
            name.as_str()
        }
    }
}

/// Builds the handler that keeps the message the visitor is writing.
///
/// The handler also watches the text before the cursor for a mention and
/// opens the list of the visitors that match it.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn draft_handler(state: ChatState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let value: String = textarea_value(&event);
        state.get_draft().set(value.clone());
        let cursor: usize = textarea_cursor(&event);
        match mention_start(&value, cursor) {
            Some(start) => {
                let filter: String = value[start + 1..cursor].to_lowercase();
                state.get_mention_filter().set(filter);
                state.get_mention_open().set(true);
                state.get_mention_index().set(0);
            }
            None => state.get_mention_open().set(false),
        }
    }))
}

/// Builds the handler that sends the message when the enter key is pressed.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The key handler.
fn key_handler(state: ChatState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard): Result<KeyboardEvent, Event> = event.clone().dyn_into() else {
            return;
        };
        if keyboard.key() != KEY_ENTER {
            return;
        }
        event.prevent_default();
        send_message(state);
    }))
}

/// Builds the handler that sends the message the visitor wrote.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn send_handler(state: ChatState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        send_message(state);
    }))
}

/// Builds the handler that writes the mentioned visitor into the message.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
/// - `String` - The name of the mentioned visitor.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn mention_handler(state: ChatState, username: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        insert_mention(state, username.as_str());
    }))
}

/// Builds the handler that uploads the file the visitor picked.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The change handler.
fn attach_handler(state: ChatState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(target): Option<EventTarget> = event.target() else {
            return;
        };
        let Ok(input): Result<HtmlInputElement, EventTarget> = target.dyn_into() else {
            return;
        };
        let Some(files): Option<FileList> = input.files() else {
            return;
        };
        let Some(file): Option<File> = files.get(0) else {
            return;
        };
        spawn_local(upload_file(state, file));
    }))
}

/// Reads the cursor of the text area the event came from.
///
/// # Arguments
///
/// - `&Event` - The event the cursor is read from.
///
/// # Returns
///
/// - `usize` - The position of the cursor inside the text.
fn textarea_cursor(event: &Event) -> usize {
    let Some(target): Option<EventTarget> = event.target() else {
        return 0;
    };
    let Ok(element): Result<HtmlTextAreaElement, EventTarget> = target.dyn_into() else {
        return 0;
    };
    element.selection_start().ok().flatten().unwrap_or(0) as usize
}

/// Finds the mention the cursor of the text area stands in.
///
/// # Arguments
///
/// - `&str` - The text of the message.
/// - `usize` - The position of the cursor inside the text.
///
/// # Returns
///
/// - `Option<usize>` - The position of the mention sign.
fn mention_start(text: &str, cursor: usize) -> Option<usize> {
    let head: &str = text.get(..cursor)?;
    let Some(at) = head.rfind('@') else {
        return None;
    };
    let filter: &str = head.get(at + 1..)?;
    if filter.chars().any(|character: char| character.is_whitespace()) {
        return None;
    }
    Some(at)
}
