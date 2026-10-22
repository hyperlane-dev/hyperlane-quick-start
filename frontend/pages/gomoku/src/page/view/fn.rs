use super::*;

/// Renders the gomoku page.
///
/// The page holds the panel of the room, the board of the game and the lists
/// of the players, of the spectators and of the moves.
///
/// # Arguments
///
/// - `VirtualNode<GomokuPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn gomoku_page(node: VirtualNode<GomokuPageProps>) -> VirtualNode {
    let GomokuPageProps { state }: GomokuPageProps = node.try_get_props().unwrap_or_default();
    let cells: Signal<Vec<GomokuCell>> = state.get_cells();
    let players: Signal<Vec<GomokuPlayer>> = state.get_players();
    let spectators: Signal<Vec<String>> = state.get_spectators();
    let moves: Signal<Vec<GomokuMove>> = state.get_moves();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "⭕"
                title: "Gomoku"
                subtitle: "Play a game of five in a row against another visitor."
            }
            if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Error
                    state.get_error().get().as_str()
                }
            }
            div {
                class: c_gomoku_layout()
                div {
                    class: c_gomoku_room_card()
                    euv_card {
                        title: TITLE_ROOM
                        div {
                            class: c_gomoku_panel()
                            div {
                                class: c_gomoku_status()
                                span {
                                    class: c_gomoku_status_dot()
                                }
                                span {
                                    class: c_gomoku_status_text()
                                    {
                                        connection_label(state)
                                    }
                                }
                            }
                            euv_field {
                                id: ID_USER
                                label: LABEL_USER
                                value: state.get_user_id()
                            }
                            euv_field {
                                id: ID_ROOM
                                label: LABEL_ROOM
                                placeholder: PLACEHOLDER_ROOM
                                value: state.get_room_id()
                                oninput: room_handler(state)
                            }
                            div {
                                class: c_button_controls()
                                euv_button {
                                    variant: EuvButtonVariant::Primary
                                    onclick: create_handler(state)
                                    LABEL_CREATE
                                }
                                euv_button {
                                    variant: EuvButtonVariant::Outline
                                    onclick: join_handler(state)
                                    LABEL_JOIN
                                }
                                euv_button {
                                    variant: EuvButtonVariant::Outline
                                    onclick: watch_handler(state)
                                    LABEL_WATCH
                                }
                                euv_button {
                                    variant: EuvButtonVariant::Outline
                                    onclick: leave_handler(state)
                                    LABEL_LEAVE
                                }
                            }
                            div {
                                class: c_gomoku_info()
                                div {
                                    class: c_gomoku_info_row()
                                    span {
                                        class: c_gomoku_info_label()
                                        LABEL_CURRENT_ROOM
                                    }
                                    span {
                                        class: c_gomoku_info_value()
                                        {
                                            room_label(state)
                                        }
                                    }
                                }
                                div {
                                    class: c_gomoku_info_row()
                                    span {
                                        class: c_gomoku_info_label()
                                        LABEL_ROLE
                                    }
                                    span {
                                        class: c_gomoku_info_value()
                                        {
                                            state.get_role().get()
                                        }
                                    }
                                }
                                div {
                                    class: c_gomoku_info_row()
                                    span {
                                        class: c_gomoku_info_label()
                                        LABEL_TURN
                                    }
                                    span {
                                        class: c_gomoku_info_value()
                                        {
                                            turn_label(state)
                                        }
                                    }
                                }
                                div {
                                    class: c_gomoku_info_row()
                                    span {
                                        class: c_gomoku_info_label()
                                        LABEL_STATUS
                                    }
                                    span {
                                        class: c_gomoku_info_value()
                                        {
                                            status_label(state)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: c_gomoku_board_card()
                    euv_card {
                        title: TITLE_BOARD
                        div {
                            class: c_gomoku_board()
                            for cell in { cells } {
                                div {
                                    key: format!("{}-{}", cell.x, cell.y)
                                    class: c_gomoku_cell()
                                    onclick: place_stone_handler(state, cell.x, cell.y)
                                    if cell.value == STONE_BLACK {
                                        div {
                                            class: c_gomoku_stone_black()
                                        }
                                    } else if cell.value == STONE_WHITE {
                                        div {
                                            class: c_gomoku_stone_white()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: c_gomoku_players_card()
                    euv_card {
                        title: TITLE_PLAYERS
                        div {
                            class: c_gomoku_list()
                            if { state.get_players().get().is_empty() } {
                                div {
                                    class: c_gomoku_list_item()
                                    TEXT_NONE
                                }
                            }
                            for player in { players } {
                                div {
                                    key: player.user_id.to_string()
                                    class: c_gomoku_list_item()
                                    span {
                                        class: c_gomoku_list_name()
                                        player.user_id.as_str()
                                    }
                                    span {
                                        class: c_gomoku_list_value()
                                        {
                                            stone_label(player.color)
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            class: c_gomoku_section_title()
                            TITLE_SPECTATORS
                        }
                        div {
                            class: c_gomoku_list()
                            if { state.get_spectators().get().is_empty() } {
                                div {
                                    class: c_gomoku_list_item()
                                    TEXT_NONE
                                }
                            }
                            for spectator in { spectators } {
                                div {
                                    key: spectator.to_string()
                                    class: c_gomoku_list_item()
                                    spectator.as_str()
                                }
                            }
                        }
                        div {
                            class: c_gomoku_section_title()
                            TITLE_MOVES
                        }
                        div {
                            class: c_gomoku_list()
                            if { state.get_moves().get().is_empty() } {
                                div {
                                    class: c_gomoku_list_item()
                                    TEXT_NONE
                                }
                            }
                            for entry in { moves } {
                                div {
                                    key: entry.step.to_string()
                                    class: c_gomoku_list_item()
                                    move_label(&entry)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Returns the name of the colour of a stone.
///
/// # Arguments
///
/// - `GomokuStone` - The colour of the stone.
///
/// # Returns
///
/// - `String` - The name of the colour.
fn stone_label(stone: GomokuStone) -> String {
    match stone {
        GomokuStone::Black => LABEL_BLACK.to_string(),
        GomokuStone::White => LABEL_WHITE.to_string(),
    }
}

/// Returns the name of the status of a room.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `String` - The name of the status.
fn status_label(state: GomokuState) -> String {
    let Some(room) = state.get_room().get() else {
        return TEXT_NONE.to_string();
    };
    match room.status {
        GomokuStatus::Waiting => LABEL_WAITING.to_string(),
        GomokuStatus::InProgress => LABEL_IN_PROGRESS.to_string(),
        GomokuStatus::Finished => LABEL_FINISHED.to_string(),
    }
}

/// Returns the identifier of the room the visitor is in.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `String` - The identifier of the room.
fn room_label(state: GomokuState) -> String {
    let Some(room) = state.get_room().get() else {
        return TEXT_NONE.to_string();
    };
    if room.room_id.is_empty() {
        return TEXT_NONE.to_string();
    }
    room.room_id
}

/// Returns the colour that plays next.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `String` - The name of the colour.
fn turn_label(state: GomokuState) -> String {
    let Some(room) = state.get_room().get() else {
        return TEXT_NONE.to_string();
    };
    stone_label(room.next_turn)
}

/// Returns the text of the state of the connection.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `String` - The text of the state of the connection.
fn connection_label(state: GomokuState) -> String {
    if state.get_connected().get() {
        return LABEL_CONNECTED.to_string();
    }
    LABEL_DISCONNECTED.to_string()
}

/// Returns the text of one move of the log.
///
/// # Arguments
///
/// - `&GomokuMove` - The move of the log.
///
/// # Returns
///
/// - `String` - The text of the move.
fn move_label(entry: &GomokuMove) -> String {
    format!(
        "#{} {} ({}, {})",
        entry.step,
        stone_label(entry.color),
        entry.x,
        entry.y
    )
}

/// Builds the handler that keeps the room the visitor asked for.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn room_handler(state: GomokuState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_room_id().set(input_value(&event));
    }))
}

/// Builds the handler that creates a room.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn create_handler(state: GomokuState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        create_room(state);
    }))
}

/// Builds the handler that joins a room.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn join_handler(state: GomokuState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        join_room(state);
    }))
}

/// Builds the handler that watches a room.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn watch_handler(state: GomokuState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spectate_room(state);
    }))
}

/// Builds the handler that leaves the room.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn leave_handler(state: GomokuState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        leave_room(state);
    }))
}

/// Builds the handler that places a stone on the board.
///
/// # Arguments
///
/// - `GomokuState` - The state the page renders from.
/// - `usize` - The column of the stone.
/// - `usize` - The row of the stone.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn place_stone_handler(state: GomokuState, x: usize, y: usize) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        place_stone(state, x, y);
    }))
}
