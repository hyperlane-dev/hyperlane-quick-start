use super::*;

/// Creates the reactive state the chat page renders from.
///
/// The state holds the messages of the room, the visitors that are online,
/// the message the visitor is writing and the list of the mentioned
/// visitors. The room is opened as soon as the page is mounted, and the
/// keep alive frame is sent every few seconds afterwards.
///
/// # Returns
///
/// - `ChatState` - The state the page renders from.
pub fn use_chat_state() -> ChatState {
    let messages: Signal<Vec<ChatMessage>> = App::use_signal(Vec::new);
    let users: Signal<Vec<OnlineUser>> = App::use_signal(Vec::new);
    let online_count: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let connected: Signal<bool> = App::use_signal(|| false);
    let username: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let uuid: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let draft: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let mention_open: Signal<bool> = App::use_signal(|| false);
    let mention_filter: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let mention_index: Signal<usize> = App::use_signal(|| 0);
    let candidates: Signal<Vec<OnlineUser>> = App::use_signal(Vec::new);
    let attempts: Signal<u32> = App::use_signal(|| 0);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let uploading: Signal<bool> = App::use_signal(|| false);
    let upload_message: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let state: ChatState = ChatState {
        messages,
        users,
        online_count,
        connected,
        username,
        uuid,
        draft,
        mention_open,
        mention_filter,
        mention_index,
        candidates,
        attempts,
        error,
        uploading,
        upload_message,
    };
    let identity: String = visitor_uuid();
    state.get_uuid().set(identity.clone());
    state
        .get_username()
        .set(format!("{USER_NAME_PREFIX}{identity}"));
    let _started: Signal<bool> = App::use_signal(|| {
        open_room(state);
        spawn_local(load_history(state));
        spawn_local(load_online_users(state));
        true
    });
    let _keep_alive: IntervalHandle =
        App::use_interval(PING_INTERVAL, move || {
            send_frame(FRAME_PING, TEXT_EMPTY);
        });
    watch!(users, mention_filter, |_users: Vec<OnlineUser>, _filter: String| {
        let filter: String = _filter;
        let candidates: Vec<OnlineUser> = _users
            .into_iter()
            .filter(|user: &OnlineUser| {
                filter.is_empty() || user.username.to_lowercase().contains(&filter)
            })
            .collect();
        state.get_candidates().set(candidates);
    });
    state
}

/// Opens the chat room and wires the handlers of its socket.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The room is open.
pub fn open_room(state: ChatState) {
    let uuid: String = state.get_uuid().get();
    let url: String = room_url(&uuid);
    let Ok(socket): Result<WebSocket, JsValue> = WebSocket::new(&url) else {
        state
            .get_error()
            .set("The chat room could not be opened".to_string());
        schedule_reconnect(state);
        return;
    };
    let on_open: Closure<dyn FnMut(Event)> = Closure::new(move |_: Event| {
        state.get_connected().set(true);
        state.get_attempts().set(0);
        state.get_error().set(TEXT_EMPTY.to_string());
        spawn_local(load_online_users(state));
    });
    let on_close: Closure<dyn FnMut(Event)> = Closure::new(move |_: Event| {
        state.get_connected().set(false);
        schedule_reconnect(state);
    });
    let on_error: Closure<dyn FnMut(Event)> = Closure::new(move |_: Event| {
        state.get_connected().set(false);
    });
    let on_message: Closure<dyn FnMut(MessageEvent)> =
        Closure::new(move |event: MessageEvent| {
            let data: JsValue = event.data();
            let Some(raw): Option<String> = data.as_string() else {
                return;
            };
            let Ok(frame): Result<ChatFrame, serde_json::Error> =
                serde_json::from_str(&raw)
            else {
                return;
            };
            handle_frame(state, frame);
        });
    socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    socket.set_onclose(Some(on_close.as_ref().unchecked_ref()));
    socket.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    let handlers: Vec<JsValue> = vec![
        on_open.into_js_value(),
        on_close.into_js_value(),
        on_error.into_js_value(),
        on_message.into_js_value(),
    ];
    CHAT_CONNECTION.with(|connection: &RefCell<Option<ChatConnection>>| {
        let previous: Option<ChatConnection> =
            connection.replace(Some(ChatConnection { socket, handlers }));
        if let Some(previous) = previous {
            drop(previous.handlers);
        }
    });
}

/// Writes one frame of the room into the state of the page.
///
/// The answer to a keep alive frame is dropped, the frame of the online
/// count updates the counter of the room, and every other frame becomes a
/// message of the room.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
/// - `ChatFrame` - The frame the room sent.
///
/// # Returns
///
/// - `()` - The frame has been written into the state.
pub fn handle_frame(state: ChatState, frame: ChatFrame) {
    if frame.kind == FRAME_PANG {
        return;
    }
    if frame.kind == FRAME_ONLINE_COUNT {
        state.get_online_count().set(frame.data.clone());
        spawn_local(load_online_users(state));
        return;
    }
    let own: bool = frame.name == state.get_username().get();
    let system: bool = frame.kind == FRAME_SYSTEM || frame.kind == FRAME_GPT_RESPONSE;
    let message: ChatMessage = ChatMessage {
        id: format!("{}-{}", frame.time, frame.name),
        sender: frame.name.clone(),
        content: frame.data.clone(),
        time: frame.time,
        own,
        system,
    };
    push_message(state, message);
}

/// Sends the message the visitor wrote to the room.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The message has been sent.
pub fn send_message(state: ChatState) {
    let draft: String = state.get_draft().get();
    let content: String = draft.trim().to_string();
    if content.is_empty() {
        return;
    }
    if !state.get_connected().get() {
        state
            .get_error()
            .set("The chat room is not connected".to_string());
        return;
    }
    if send_frame(FRAME_TEXT, &content) {
        state.get_draft().set(TEXT_EMPTY.to_string());
        state.get_mention_open().set(false);
        state.get_error().set(TEXT_EMPTY.to_string());
    }
}

/// Sends one frame to the room.
///
/// # Arguments
///
/// - `&str` - The type of the frame.
/// - `&str` - The payload of the frame.
///
/// # Returns
///
/// - `bool` - Whether the frame has been sent.
pub fn send_frame(kind: &str, data: &str) -> bool {
    let frame: ChatOutgoing = ChatOutgoing {
        kind: kind.to_string(),
        data: data.to_string(),
    };
    let Ok(encoded): Result<String, serde_json::Error> = serde_json::to_string(&frame) else {
        return false;
    };
    CHAT_CONNECTION.with(|connection: &RefCell<Option<ChatConnection>>| {
        let borrowed: Option<ChatConnection> = connection.borrow().clone();
        let Some(chat) = borrowed else {
            return false;
        };
        chat.socket.send_with_str(&encoded).is_ok()
    })}

/// Loads the history of the room and writes it into the state.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The history has been written into the state.
pub async fn load_history(state: ChatState) {
    let path: String = format!("{PATH_HISTORY}?limit={HISTORY_LIMIT}");
    match get_json::<ChatHistory>(&path).await {
        Ok(history) => {
            let mut messages: Vec<ChatMessage> = Vec::new();
            for message in history.messages {
                messages.push(ChatMessage {
                    id: format!("history-{}", message.id),
                    sender: message.sender_name.clone(),
                    content: message.content.clone(),
                    time: message.created_at,
                    own: message.session_id == state.get_uuid().get(),
                    system: message.sender_type != "user",
                });
            }
            let mut known: Vec<ChatMessage> = state.get_messages().get();
            known.extend(messages);
            state.get_messages().set(known);
        }
        Err(error) => state.get_error().set(error.message),
    }
}

/// Loads the visitors that are online and writes them into the state.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The visitors have been written into the state.
pub async fn load_online_users(state: ChatState) {
    match get_json::<OnlineUserList>(PATH_USERS_ONLINE).await {
        Ok(list) => state.get_users().set(list.users),
        Err(error) => state.get_error().set(error.message),
    }
}

/// Writes the name of a mentioned visitor into the message being written.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
/// - `&str` - The name of the mentioned visitor.
///
/// # Returns
///
/// - `()` - The message has been written.
pub fn insert_mention(state: ChatState, username: &str) {
    let draft: String = state.get_draft().get();
    let Some(at) = draft.rfind('@') else {
        state.get_mention_open().set(false);
        return;
    };
    let mut next: String = String::from(&draft[..at]);
    next.push('@');
    next.push_str(username);
    next.push(' ');
    state.get_draft().set(next);
    state.get_mention_open().set(false);
    state.get_mention_filter().set(TEXT_EMPTY.to_string());
    state.get_mention_index().set(0);
}

/// Uploads a file and sends its address to the room.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
/// - `File` - The file to upload.
///
/// # Returns
///
/// - `()` - The file has been uploaded and its address has been sent.
pub async fn upload_file(state: ChatState, file: File) {
    state.get_uploading().set(true);
    state.get_upload_message().set(TEXT_EMPTY.to_string());
    let id: String = new_upload_id();
    let total_chunks: i32 = (file.size() / CHUNK_SIZE as f64).ceil() as i32;
    let total_chunks: i32 = total_chunks.max(1);
    let total_chunks_text: String = total_chunks.to_string();
    let encoded_name: String = encode_uri_component(&file.name())
        .as_string()
        .unwrap_or_default();
    let register_headers: Vec<(&str, &str)> = vec![
        (HEADER_FILE_ID, id.as_str()),
        (HEADER_TOTAL_CHUNKS, total_chunks_text.as_str()),
        (HEADER_FILE_NAME, encoded_name.as_str()),
    ];
    if let Err(error) = upload_request(METHOD_POST, PATH_UPLOAD_REGISTER, &register_headers, None).await {
        fail_upload(state, error.message);
        return;
    }
    for index in 0..total_chunks {
        let start: i32 = index * CHUNK_SIZE;
        let end: i32 = start + CHUNK_SIZE;
        let chunk: Blob = match file.slice_with_i32_and_i32(start, end) {
            Ok(chunk) => chunk,
            Err(_) => {
                fail_upload(state, "The file could not be read".to_string());
                return;
            }
        };
        let chunk_index_text: String = index.to_string();
        let chunk_headers: Vec<(&str, &str)> = vec![
            (HEADER_FILE_ID, id.as_str()),
            (HEADER_CHUNK_INDEX, chunk_index_text.as_str()),
        ];
        if let Err(error) =
            upload_request(METHOD_POST, PATH_UPLOAD_SAVE, &chunk_headers, Some(chunk)).await
        {
            fail_upload(state, error.message);
            return;
        }
    }
    let merge_headers: Vec<(&str, &str)> = vec![(HEADER_FILE_ID, id.as_str())];
    match upload_request(METHOD_POST, PATH_UPLOAD_MERGE, &merge_headers, None).await {
        Ok(response) => {
            state.get_uploading().set(false);
            state
                .get_upload_message()
                .set(format!("{} has been uploaded", file.name()));
            send_frame(FRAME_TEXT, &response.url);
        }
        Err(error) => fail_upload(state, error.message),
    }
}

/// Sends one request to one of the upload endpoints.
///
/// The upload endpoints answer with a flat object instead of the envelope of
/// the other endpoints, so the body is read as text and parsed here.
///
/// # Arguments
///
/// - `&str` - The HTTP method the request is issued with.
/// - `&str` - The path the request is sent to.
/// - `&[(&str, &str)]` - The headers the request carries.
/// - `Option<Blob>` - The body the request carries.
///
/// # Returns
///
/// - `Result<UploadAnswer, ApiError>` - The answer of the endpoint, or the failure.
pub async fn upload_request(
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: Option<Blob>,
) -> Result<UploadAnswer, ApiError> {
    let Some(window): Option<Window> = window() else {
        return Err(ApiError {
            code: 0,
            message: MESSAGE_NO_WINDOW.to_string(),
        });
    };
    let header_map: Headers =
        Headers::new().map_err(|error: JsValue| transport_failure(error))?;
    for (name, value) in headers {
        header_map
            .set(name, value)
            .map_err(|error: JsValue| transport_failure(error))?;
    }
    let init: RequestInit = RequestInit::new();
    init.set_method(method);
    init.set_mode(RequestMode::SameOrigin);
    init.set_credentials(RequestCredentials::Include);
    init.set_headers(&JsValue::from(header_map));
    if let Some(body) = body {
        init.set_body(&JsValue::from(body));
    }
    let promise: Promise = window.fetch_with_str_and_init(path, &init);
    let value: JsValue = JsFuture::from(promise)
        .await
        .map_err(|error: JsValue| transport_failure(error))?;
    let response: Response = value
        .dyn_into()
        .map_err(|error: JsValue| transport_failure(error))?;
    let text_promise: Promise = response
        .text()
        .map_err(|error: JsValue| transport_failure(error))?;
    let answer: JsValue = JsFuture::from(text_promise)
        .await
        .map_err(|error: JsValue| transport_failure(error))?;
    let raw: String = answer.as_string().unwrap_or_default();
    let parsed: UploadAnswer =
        serde_json::from_str(&raw).map_err(|error: serde_json::Error| ApiError {
            code: 0,
            message: format!("{MESSAGE_INVALID_PAYLOAD}{MESSAGE_SEPARATOR}{error}"),
        })?;
    if parsed.code != STATUS_SUCCESS {
        return Err(ApiError {
            code: parsed.code,
            message: parsed.msg,
        });
    }
    Ok(parsed)
}

/// Adds one message to the room.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
/// - `ChatMessage` - The message to add.
///
/// # Returns
///
/// - `()` - The message has been added.
fn push_message(state: ChatState, message: ChatMessage) {
    let mut messages: Vec<ChatMessage> = state.get_messages().get();
    messages.push(message);
    state.get_messages().set(messages);
}

/// Schedules the next attempt to open the chat room.
///
/// The delay grows with every failed attempt, so a server that stays down
/// is not asked for a connection on every tick.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The attempt has been scheduled.
fn schedule_reconnect(state: ChatState) {
    let attempts: u32 = state.get_attempts().get() + 1;
    state.get_attempts().set(attempts);
    let delay: i32 = reconnect_delay(attempts);
    let callback: JsValue = Closure::once_into_js(move || {
        open_room(state);
    });
    let Some(window): Option<Window> = window() else {
        return;
    };
    let function: &js_sys::Function = callback.unchecked_ref::<js_sys::Function>();
    let _scheduled: Result<i32, JsValue> =
        window.set_timeout_with_callback_and_timeout_and_arguments_0(function, delay);
}

/// Builds the delay of the next attempt to open the chat room.
///
/// # Arguments
///
/// - `u32` - The number of the failed attempts.
///
/// # Returns
///
/// - `i32` - The delay of the attempt in milliseconds.
fn reconnect_delay(attempts: u32) -> i32 {
    let steps: u32 = attempts / RECONNECT_STEP;
    let delay: i64 = RECONNECT_BASE_DELAY as i64 * 2_i64.pow(steps);
    delay.min(RECONNECT_MAX_DELAY as i64) as i32
}

/// Builds the address of the chat room.
///
/// # Arguments
///
/// - `&str` - The identity of the visitor.
///
/// # Returns
///
/// - `String` - The address of the room.
fn room_url(uuid: &str) -> String {
    let Some(window): Option<Window> = window() else {
        return String::new();
    };
    let location: Location = window.location();
    let protocol: String = location.protocol().unwrap_or_default();
    let scheme: &str = if protocol == "https:" { "wss" } else { "ws" };
    let host: String = location.host().unwrap_or_default();
    format!("{scheme}://{host}{PATH_CHAT}?uuid={uuid}")
}

/// Reads the identity of the visitor out of the browser storage.
///
/// The identity is created the first time the visitor opens the page and
/// is kept afterwards, so the room recognises the visitor across reloads.
///
/// # Returns
///
/// - `String` - The identity of the visitor.
pub fn visitor_uuid() -> String {
    let Some(window): Option<Window> = window() else {
        return String::new();
    };
    let Some(storage): Option<Storage> = window.local_storage().ok().flatten() else {
        return new_upload_id();
    };
    if let Some(known) = storage.get_item(STORAGE_UUID).ok().flatten() {
        if !known.is_empty() {
            return known;
        }
    }
    let identity: String = new_upload_id();
    let _stored: Result<(), JsValue> = storage.set_item(STORAGE_UUID, &identity);
    identity
}

/// Builds the identifier of one upload session.
///
/// # Returns
///
/// - `String` - The identifier of the upload session.
fn new_upload_id() -> String {
    let stamp: i64 = Date::now() as i64;
    let noise: f64 = js_sys::Math::random();
    format!("{stamp:x}{:x}", (noise * 1_000_000.0) as u64)
}

/// Marks the upload of a file as failed.
///
/// # Arguments
///
/// - `ChatState` - The state the page renders from.
/// - `String` - The message of the failure.
///
/// # Returns
///
/// - `()` - The state has been written.
fn fail_upload(state: ChatState, message: String) {
    state.get_uploading().set(false);
    state.get_upload_message().set(message);
}

/// Builds the failure reported when the browser could not complete a request.
///
/// # Arguments
///
/// - `JsValue` - The value the browser reported.
///
/// # Returns
///
/// - `ApiError` - The failure describing the transport level problem.
fn transport_failure(error: JsValue) -> ApiError {
    let cause: String = error.as_string().unwrap_or_else(|| format!("{error:?}"));
    ApiError {
        code: 0,
        message: format!("{MESSAGE_REQUEST_FAILED}{MESSAGE_SEPARATOR}{cause}"),
    }
}

/// Formats the moment a message was sent as a short local time.
///
/// # Arguments
///
/// - `i64` - The moment in milliseconds since the Unix epoch.
///
/// # Returns
///
/// - `String` - The local time of the moment.
pub fn format_time(timestamp: i64) -> String {
    if timestamp <= 0 {
        return TEXT_EMPTY.to_string();
    }
    let date: Date = Date::new(&JsValue::from_f64(timestamp as f64));
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date(),
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds()
    )
}
