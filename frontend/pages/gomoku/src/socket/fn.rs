use super::*;

/// Builds the address of the WebSocket of the page.
///
/// The address carries the identifier of the visitor, so the server knows
/// which player the frames belong to.
///
/// # Returns
///
/// - `String` - The address of the WebSocket.
pub fn socket_url() -> String {
    let Some(window): Option<Window> = window() else {
        return String::new();
    };
    let location: Location = window.location();
    let protocol: &str = if location.protocol().unwrap_or_default() == "https:" {
        "wss"
    } else {
        "ws"
    };
    let host: String = location.host().unwrap_or_default();
    format!(
        "{protocol}://{host}{PATH_GOMOKU}?{QUERY_UID}={}",
        user_id()
    )
}

/// Returns the identifier of the visitor.
///
/// The identifier is kept in the storage of the browser, so the visitor keeps
/// the same seat in a room across a reload of the page.
///
/// # Returns
///
/// - `String` - The identifier of the visitor.
pub fn user_id() -> String {
    let Some(window): Option<Window> = window() else {
        return generated_user_id();
    };
    let Ok(Some(storage)): Result<Option<Storage>, JsValue> = window.local_storage() else {
        return generated_user_id();
    };
    if let Ok(Some(existing)) = storage.get_item(STORAGE_USER_ID)
        && !existing.is_empty()
    {
        return existing;
    }
    let generated: String = generated_user_id();
    let _: Result<(), JsValue> = storage.set_item(STORAGE_USER_ID, &generated);
    generated
}

/// Connects to the WebSocket of the page.
///
/// The client connects again when the server closes the connection, until it
/// ran out of attempts.
///
/// # Arguments
///
/// - `Rc<dyn Fn(GomokuFrame)>` - The handler of the frames the server sends.
/// - `Rc<dyn Fn(bool)>` - The handler of the state of the connection.
///
/// # Returns
///
/// - `()` - The client is connected, or the browser refused the connection.
pub fn connect(on_frame: Rc<dyn Fn(GomokuFrame)>, on_connected: Rc<dyn Fn(bool)>) {
    let url: String = socket_url();
    let Ok(socket): Result<WebSocket, JsValue> = WebSocket::new(&url) else {
        return;
    };
    SOCKET.with(|slot: &RefCell<Option<WebSocket>>| {
        let mut guard: RefMut<'_, Option<WebSocket>> = slot.borrow_mut();
        *guard = Some(socket.clone());
    });
    let on_connected_open: Rc<dyn Fn(bool)> = on_connected.clone();
    let on_open: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        RECONNECT_ATTEMPTS.with(|attempts: &Cell<u32>| {
            attempts.set(0);
        });
        on_connected_open(true);
    }));
    socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();
    let on_frame_message: Rc<dyn Fn(GomokuFrame)> = on_frame.clone();
    let on_message: Closure<dyn FnMut(MessageEvent)> =
        Closure::wrap(Box::new(move |event: MessageEvent| {
            let Some(text): Option<String> = event.data().as_string() else {
                return;
            };
            let Ok(frame): Result<GomokuFrame, serde_json::Error> = serde_json::from_str(&text)
            else {
                return;
            };
            on_frame_message(frame);
        }));
    socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
    let on_close: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        SOCKET.with(|slot: &RefCell<Option<WebSocket>>| {
            let mut guard: RefMut<'_, Option<WebSocket>> = slot.borrow_mut();
            *guard = None;
        });
        on_connected(false);
        schedule_reconnect(on_frame.clone(), on_connected.clone());
    }));
    socket.set_onclose(Some(on_close.as_ref().unchecked_ref()));
    on_close.forget();
}

/// Sends a frame to the server.
///
/// The frame is dropped when the client is not connected, so the visitor has
/// to wait for the connection to come back.
///
/// # Arguments
///
/// - `&str` - The kind of the frame.
/// - `&str` - The room the frame belongs to.
/// - `serde_json::Value` - The payload of the frame.
///
/// # Returns
///
/// - `()` - The frame has been handed to the socket of the browser.
pub fn send(kind: &str, room_id: &str, payload: serde_json::Value) {
    let frame: serde_json::Value = serde_json::json!({
        "type": kind,
        "room_id": room_id,
        "payload": payload,
    });
    let Ok(text): Result<String, serde_json::Error> = serde_json::to_string(&frame) else {
        return;
    };
    SOCKET.with(|slot: &RefCell<Option<WebSocket>>| {
        let socket: Option<WebSocket> = slot.borrow().as_ref().cloned();
        let Some(socket) = socket else {
            return;
        };
        if socket.ready_state() == WebSocket::OPEN {
            let _: Result<(), JsValue> = socket.send_with_str(&text);
        }
    });
}

/// Connects again after the server closed the connection.
///
/// The client waits longer after every attempt, and gives up when it ran out
/// of attempts.
///
/// # Arguments
///
/// - `Rc<dyn Fn(GomokuFrame)>` - The handler of the frames the server sends.
/// - `Rc<dyn Fn(bool)>` - The handler of the state of the connection.
///
/// # Returns
///
/// - `()` - The client tries again later, or it gave up.
fn schedule_reconnect(on_frame: Rc<dyn Fn(GomokuFrame)>, on_connected: Rc<dyn Fn(bool)>) {
    let attempts: u32 = RECONNECT_ATTEMPTS.with(|cell: &Cell<u32>| {
        let next: u32 = cell.get() + 1;
        cell.set(next);
        next
    });
    if attempts > MAX_RECONNECT_ATTEMPTS {
        return;
    }
    let Some(window): Option<Window> = window() else {
        return;
    };
    let delay: i32 = (RECONNECT_INTERVAL_MS * attempts) as i32;
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        connect(on_frame.clone(), on_connected.clone());
    }));
    let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        delay,
    );
    closure.forget();
}

/// Builds the identifier of a visitor that visits the page for the first
/// time.
///
/// # Returns
///
/// - `String` - The identifier of the visitor.
fn generated_user_id() -> String {
    let moment: u64 = js_sys::Date::now() as u64;
    let noise: u64 = (js_sys::Math::random() * 1_000_000.0) as u64;
    format!(
        "{PREFIX_USER_ID}{}_{}",
        to_base(moment),
        to_base(noise)
    )
}

/// Writes a number in the base the identifiers of the visitors use.
///
/// # Arguments
///
/// - `u64` - The number to write.
///
/// # Returns
///
/// - `String` - The number in the base of the identifiers.
fn to_base(value: u64) -> String {
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    if value == 0 {
        return "0".to_string();
    }
    let mut digits: Vec<u8> = Vec::new();
    let mut rest: u64 = value;
    while rest > 0 {
        digits.push(DIGITS[(rest % DIGITS_BASE as u64) as usize]);
        rest /= DIGITS_BASE as u64;
    }
    digits.reverse();
    String::from_utf8(digits).unwrap_or_default()
}
