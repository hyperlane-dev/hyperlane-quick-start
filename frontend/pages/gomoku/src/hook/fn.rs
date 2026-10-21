use super::*;

/// Creates the reactive state the gomoku page renders from.
///
/// The state holds the room the visitor is in, the board of the room and the
/// lists of the players, of the spectators and of the moves. The client
/// connects to the server as soon as the page is mounted.
///
/// # Returns
///
/// - `GomokuState` - The state the page renders from.
pub fn use_gomoku_state() -> GomokuState {
    let user_id: Signal<String> = App::use_signal(user_id);
    let room_id: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let room: Signal<Option<GomokuRoom>> = App::use_signal(|| None);
    let cells: Signal<Vec<GomokuCell>> = App::use_signal(Vec::new);
    let players: Signal<Vec<GomokuPlayer>> = App::use_signal(Vec::new);
    let spectators: Signal<Vec<String>> = App::use_signal(Vec::new);
    let moves: Signal<Vec<GomokuMove>> = App::use_signal(Vec::new);
    let role: Signal<String> = App::use_signal(|| ROLE_NONE.to_string());
    let connected: Signal<bool> = App::use_signal(|| false);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let state: GomokuState = GomokuState {
        user_id,
        room_id,
        room,
        cells,
        players,
        spectators,
        moves,
        role,
        connected,
        error,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        let on_frame: Rc<dyn Fn(GomokuFrame)> = Rc::new(move |frame: GomokuFrame| {
            handle_frame(state, frame);
        });
        let on_connected: Rc<dyn Fn(bool)> = Rc::new(move |is_connected: bool| {
            state.get_connected().set(is_connected);
        });
        connect(on_frame, on_connected);
        true
    });
    state
}

/// Handles one frame the server sent.
///
/// The state of the room is written into the page, and a failure is shown as
/// the message of the page.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
/// - `GomokuFrame` - The frame the server sent.
///
/// # Returns
///
/// - `()` - The frame has been written into the state.
pub fn handle_frame(state: GomokuState, frame: GomokuFrame) {
    match frame.r#type.as_str() {
        TYPE_ROOM_STATE => {
            if let Ok(room) = serde_json::from_value::<GomokuRoom>(frame.payload) {
                apply_room(state, room);
            }
        }
        TYPE_MOVE_RESULT => {
            let Some(payload) = frame.payload.get(KEY_ROOM) else {
                return;
            };
            if let Ok(room) = serde_json::from_value::<GomokuRoom>(payload.clone()) {
                apply_room(state, room);
            }
        }
        TYPE_ERROR => {
            let message: String = frame
                .payload
                .get(KEY_MESSAGE)
                .and_then(|value: &serde_json::Value| value.as_str())
                .unwrap_or(MESSAGE_UNKNOWN_ERROR)
                .to_string();
            state.get_error().set(message);
        }
        _ => {}
    }
}

/// Writes the state of a room into the page.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
/// - `GomokuRoom` - The room the server reported.
///
/// # Returns
///
/// - `()` - The room has been written into the state.
fn apply_room(state: GomokuState, room: GomokuRoom) {
    let user: String = state.get_user_id().get();
    let role: &str = if room.players
        .iter()
        .any(|player: &GomokuPlayer| player.user_id == user) {
        ROLE_PLAYER
    } else if room.spectators
        .iter()
        .any(|spectator: &String| spectator == &user) {
        ROLE_SPECTATOR
    } else {
        ROLE_NONE
    };
    let cells: Vec<GomokuCell> = build_cells(&room.board);
    let mut moves: Vec<GomokuMove> = room.moves.clone();
    moves.reverse();
    state.get_cells().set(cells);
    state.get_players().set(room.players.clone());
    state.get_spectators().set(room.spectators.clone());
    state.get_moves().set(moves);
    state.get_role().set(role.to_string());
    state.get_room().set(Some(room));
    state.get_error().set(TEXT_EMPTY.to_string());
}

/// Builds the cells of the board of a room.
///
/// The board of the server holds one row per line of the board, so the cells
/// are built in the order the board is played in.
///
/// # Arguments
///
/// - `&Vec<Vec<u8>>` - The board of the room.
///
/// # Returns
///
/// - `Vec<GomokuCell>` - The cells of the board.
fn build_cells(board: &Vec<Vec<u8>>) -> Vec<GomokuCell> {
    let mut cells: Vec<GomokuCell> = Vec::with_capacity(BOARD_COLUMNS * BOARD_ROWS);
    for y in 0..BOARD_ROWS {
        for x in 0..BOARD_COLUMNS {
            let value: u8 = board
                .get(y)
                .and_then(|row: &Vec<u8>| row.get(x))
                .copied()
                .unwrap_or(STONE_EMPTY);
            cells.push(GomokuCell { x, y, value });
        }
    }
    cells
}

/// Creates a room.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The frame has been sent to the server.
pub fn create_room(state: GomokuState) {
    let room: String = state.get_room_id().get();
    send(TYPE_CREATE_ROOM, room.trim(), serde_json::json!({}));
}

/// Joins a room.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The frame has been sent to the server.
pub fn join_room(state: GomokuState) {
    let room: String = state.get_room_id().get();
    let room: &str = room.trim();
    if room.is_empty() {
        return;
    }
    send(TYPE_JOIN_ROOM, room, serde_json::json!({}));
}

/// Watches a room.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The frame has been sent to the server.
pub fn spectate_room(state: GomokuState) {
    let room: String = state.get_room_id().get();
    let room: &str = room.trim();
    if room.is_empty() {
        return;
    }
    send(TYPE_SPECTATE, room, serde_json::json!({}));
}

/// Leaves the room the visitor is in.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The frame has been sent to the server.
pub fn leave_room(state: GomokuState) {
    let room: String = state
        .get_room()
        .get()
        .map(|current: GomokuRoom| current.room_id)
        .unwrap_or_default();
    send(TYPE_LEAVE, &room, serde_json::json!({}));
}

/// Places a stone on the board of the room.
///
/// The stone is only placed when the game is being played, when the visitor
/// plays in the room and when it is the turn of the colour of the visitor.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
/// - `usize` - The column of the stone.
/// - `usize` - The row of the stone.
///
/// # Returns
///
/// - `()` - The frame has been sent to the server.
pub fn place_stone(state: GomokuState, x: usize, y: usize) {
    let Some(room) = state.get_room().get() else {
        return;
    };
    if room.status != GomokuStatus::InProgress {
        return;
    }
    if state.get_role().get() != ROLE_PLAYER {
        return;
    }
    let user: String = state.get_user_id().get();
    let color: Option<GomokuStone> = room
        .players
        .iter()
        .find(|player: &&GomokuPlayer| player.user_id == user)
        .map(|player: &GomokuPlayer| player.color);
    if let Some(color) = color
        && color != room.next_turn
    {
        return;
    }
    send(
        TYPE_PLACE_STONE,
        &room.room_id,
        serde_json::json!({ "x": x, "y": y }),
    );
}
