use super::*;

fn two_player_room() -> GomokuRoom {
    let mut room: GomokuRoom = GomokuDomain::create_room("room-alpha", "alice");
    let assigned: StoneColor = GomokuDomain::add_player(&mut room, "bob").expect("bob joins");
    assert_eq!(assigned, StoneColor::White);
    GomokuDomain::start_game(&mut room).expect("game starts");
    room
}

fn alternate(room: &mut GomokuRoom, black: (usize, usize), white: (usize, usize)) {
    GomokuDomain::place_stone(room, "alice", black.0, black.1).expect("black move");
    GomokuDomain::place_stone(room, "bob", white.0, white.1).expect("white move");
}

#[test]
fn create_room_seats_owner_as_black_with_empty_board() {
    let room: GomokuRoom = GomokuDomain::create_room("room-beta", "carol");
    assert_eq!(room.get_room_id(), "room-beta");
    assert_eq!(room.get_owner_id(), "carol");
    assert_eq!(room.get_players().len(), 1);
    assert_eq!(room.get_players()[0].get_user_id(), "carol");
    assert_eq!(room.get_players()[0].get_color(), &StoneColor::Black);
    assert_eq!(room.get_status(), &GameStatus::Waiting);
    assert_eq!(room.get_next_turn(), &StoneColor::Black);
    assert!(room.get_spectators().is_empty());
    assert!(room.get_moves().is_empty());
    assert!(room.try_get_winner().is_none());
    assert_eq!(room.get_board().len(), 15);
    for row in room.get_board().iter() {
        assert_eq!(row.len(), 15);
        assert!(row.iter().all(|cell: &u8| *cell == 0));
    }
}

#[test]
fn create_room_accepts_empty_identifiers() {
    let room: GomokuRoom = GomokuDomain::create_room("", "");
    assert_eq!(room.get_room_id(), "");
    assert_eq!(room.get_owner_id(), "");
    assert_eq!(room.get_players().len(), 1);
    assert_eq!(room.get_players()[0].get_user_id(), "");
}

#[test]
fn add_player_returns_black_for_an_empty_room() {
    let mut room: GomokuRoom = GomokuRoom::default();
    let assigned: StoneColor = GomokuDomain::add_player(&mut room, "dave").expect("first joins");
    assert_eq!(assigned, StoneColor::Black);
    assert_eq!(room.get_players().len(), 1);
}

#[test]
fn add_player_returns_existing_color_for_a_seated_user() {
    let mut room: GomokuRoom = GomokuDomain::create_room("room-gamma", "alice");
    let assigned: StoneColor = GomokuDomain::add_player(&mut room, "alice").expect("rejoin");
    assert_eq!(assigned, StoneColor::Black);
    assert_eq!(room.get_players().len(), 1);
    let assigned_second: StoneColor = GomokuDomain::add_player(&mut room, "bob").expect("joins");
    assert_eq!(assigned_second, StoneColor::White);
    let assigned_repeat: StoneColor = GomokuDomain::add_player(&mut room, "bob").expect("rejoin");
    assert_eq!(assigned_repeat, StoneColor::White);
    assert_eq!(room.get_players().len(), 2);
}

#[test]
fn add_player_rejects_a_third_seat() {
    let mut room: GomokuRoom = GomokuDomain::create_room("room-delta", "alice");
    let _: StoneColor = GomokuDomain::add_player(&mut room, "bob").expect("bob joins");
    let result: Result<StoneColor, String> = GomokuDomain::add_player(&mut room, "erin");
    assert_eq!(result.err(), Some(ERROR_ROOM_IS_FULL.to_string()));
    assert_eq!(room.get_players().len(), 2);
}

#[test]
fn add_spectator_seats_only_unseated_users_once() {
    let mut room: GomokuRoom = GomokuDomain::create_room("room-epsilon", "alice");
    assert!(GomokuDomain::add_spectator(&mut room, "alice").eq(&false));
    assert!(room.get_spectators().is_empty());
    assert!(GomokuDomain::add_spectator(&mut room, "watcher"));
    assert_eq!(room.get_spectators(), &vec!["watcher".to_string()]);
    assert!(!GomokuDomain::add_spectator(&mut room, "watcher"));
    assert_eq!(room.get_spectators().len(), 1);
    assert!(GomokuDomain::add_spectator(&mut room, "second-watcher"));
    assert_eq!(room.get_spectators().len(), 2);
}

#[test]
fn remove_user_removes_player_and_spectator_without_finishing_a_waiting_room() {
    let mut room: GomokuRoom = GomokuDomain::create_room("room-zeta", "alice");
    let _: StoneColor = GomokuDomain::add_player(&mut room, "bob").expect("bob joins");
    assert!(GomokuDomain::add_spectator(&mut room, "watcher"));
    assert!(GomokuDomain::remove_user(&mut room, "bob"));
    assert_eq!(room.get_players().len(), 1);
    assert_eq!(room.get_status(), &GameStatus::Waiting);
    assert!(room.try_get_winner().is_none());
    assert!(GomokuDomain::remove_user(&mut room, "watcher"));
    assert!(room.get_spectators().is_empty());
    assert_eq!(room.get_status(), &GameStatus::Waiting);
    assert!(!GomokuDomain::remove_user(&mut room, "nobody"));
}

#[test]
fn remove_user_finishes_an_in_progress_game_with_the_remaining_player_as_winner() {
    let mut room: GomokuRoom = two_player_room();
    let _: Result<GomokuPlaceResult, String> = GomokuDomain::place_stone(&mut room, "alice", 0, 0);
    assert!(GomokuDomain::remove_user(&mut room, "bob"));
    assert_eq!(room.get_players().len(), 1);
    assert_eq!(room.get_status(), &GameStatus::Finished);
    assert_eq!(*room.try_get_winner(), Some(StoneColor::Black));
    assert!(!GomokuDomain::remove_user(&mut room, "bob"));
}

#[test]
fn removing_a_spectator_during_play_leaves_the_game_in_progress() {
    let mut room: GomokuRoom = two_player_room();
    let _: Result<GomokuPlaceResult, String> = GomokuDomain::place_stone(&mut room, "alice", 0, 0);
    assert!(GomokuDomain::add_spectator(&mut room, "watcher"));
    assert!(GomokuDomain::remove_user(&mut room, "watcher"));
    assert!(room.get_spectators().is_empty());
    assert_eq!(room.get_players().len(), 2);
    assert_eq!(room.get_status(), &GameStatus::InProgress);
    assert!(room.try_get_winner().is_none());
    assert_eq!(room.get_next_turn(), &StoneColor::White);
    assert_eq!(room.get_moves().len(), 1);
    assert_eq!(room.get_board()[0][0], StoneColor::Black.to_value());
}

#[test]
fn removing_an_absent_user_during_play_leaves_the_game_in_progress() {
    let mut room: GomokuRoom = two_player_room();
    assert!(!GomokuDomain::remove_user(&mut room, "nobody"));
    assert_eq!(room.get_players().len(), 2);
    assert!(room.get_spectators().is_empty());
    assert_eq!(room.get_status(), &GameStatus::InProgress);
    assert!(room.try_get_winner().is_none());
    assert_eq!(room.get_next_turn(), &StoneColor::Black);
    assert!(room.get_moves().is_empty());
}

#[test]
fn start_game_requires_exactly_two_players() {
    let mut waiting: GomokuRoom = GomokuDomain::create_room("room-eta", "alice");
    let result: Result<(), String> = GomokuDomain::start_game(&mut waiting);
    assert_eq!(
        result.err(),
        Some(ERROR_WAITING_FOR_SECOND_PLAYER.to_string())
    );
    assert_eq!(waiting.get_status(), &GameStatus::Waiting);
    let _: StoneColor = GomokuDomain::add_player(&mut waiting, "bob").expect("bob joins");
    let started: Result<(), String> = GomokuDomain::start_game(&mut waiting);
    assert_eq!(started, Ok(()));
    assert_eq!(waiting.get_status(), &GameStatus::InProgress);
}

#[test]
fn start_game_rejects_a_room_with_more_than_two_players() {
    let mut room: GomokuRoom = GomokuDomain::create_room("room-theta", "alice");
    let _: StoneColor = GomokuDomain::add_player(&mut room, "bob").expect("bob joins");
    let mut third: GomokuPlayer = GomokuPlayer::default();
    third
        .set_user_id("erin".to_string())
        .set_color(StoneColor::Black);
    room.get_mut_players().push(third);
    let result: Result<(), String> = GomokuDomain::start_game(&mut room);
    assert_eq!(
        result.err(),
        Some(ERROR_WAITING_FOR_SECOND_PLAYER.to_string())
    );
    assert_eq!(room.get_status(), &GameStatus::Waiting);
}

#[test]
fn ensure_board_replaces_a_board_with_the_wrong_row_count() {
    let mut room: GomokuRoom = GomokuRoom::default();
    room.get_mut_board().push(vec![0u8; 15]);
    GomokuDomain::ensure_board(&mut room);
    assert_eq!(room.get_board().len(), 15);
    for row in room.get_board().iter() {
        assert_eq!(row.len(), 15);
    }
}

#[test]
fn ensure_board_replaces_a_board_with_the_wrong_column_count() {
    let mut room: GomokuRoom = GomokuRoom::default();
    let mut rows: Vec<Vec<u8>> = vec![vec![0u8; 15]; 15];
    rows[7] = vec![0u8; 3];
    *room.get_mut_board() = rows;
    GomokuDomain::ensure_board(&mut room);
    assert_eq!(room.get_board().len(), 15);
    assert_eq!(room.get_board()[7].len(), 15);
}

#[test]
fn ensure_board_preserves_a_stone_on_a_valid_board() {
    let mut room: GomokuRoom = GomokuDomain::create_room("room-iota", "alice");
    room.get_mut_board()[4][6] = StoneColor::White.to_value();
    GomokuDomain::ensure_board(&mut room);
    assert_eq!(room.get_board()[4][6], StoneColor::White.to_value());
    assert_eq!(room.get_board()[0][0], 0);
}

#[test]
fn place_stone_rejects_a_move_before_the_game_starts() {
    let mut room: GomokuRoom = GomokuDomain::create_room("room-kappa", "alice");
    let result: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "alice", 0, 0);
    assert_eq!(result.err(), Some(ERROR_GAME_NOT_IN_PROGRESS.to_string()));
    assert!(room.get_moves().is_empty());
    assert_eq!(room.get_board()[0][0], 0);
}

#[test]
fn place_stone_rejects_a_user_outside_the_room() {
    let mut room: GomokuRoom = two_player_room();
    let result: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "mallory", 0, 0);
    assert_eq!(result.err(), Some(ERROR_PLAYER_NOT_FOUND.to_string()));
    assert!(room.get_moves().is_empty());
}

#[test]
fn place_stone_rejects_a_spectator_placing_a_stone() {
    let mut room: GomokuRoom = two_player_room();
    assert!(GomokuDomain::add_spectator(&mut room, "watcher"));
    let result: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "watcher", 0, 0);
    assert_eq!(result.err(), Some(ERROR_PLAYER_NOT_FOUND.to_string()));
}

#[test]
fn place_stone_rejects_a_move_out_of_turn() {
    let mut room: GomokuRoom = two_player_room();
    let result: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "bob", 1, 0);
    assert_eq!(result.err(), Some(ERROR_NOT_YOUR_TURN.to_string()));
    assert!(room.get_moves().is_empty());
    let black: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "alice", 0, 0);
    assert!(black.is_ok());
    let white_early: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "alice", 1, 0);
    assert_eq!(white_early.err(), Some(ERROR_NOT_YOUR_TURN.to_string()));
}

#[test]
fn place_stone_rejects_coordinates_outside_the_board() {
    let mut room: GomokuRoom = two_player_room();
    let _: Result<GomokuPlaceResult, String> = GomokuDomain::place_stone(&mut room, "alice", 0, 0);
    let row_overflow: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "bob", 0, 15);
    assert_eq!(row_overflow.err(), Some(ERROR_INVALID_POSITION.to_string()));
    let column_overflow: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "bob", 15, 0);
    assert_eq!(
        column_overflow.err(),
        Some(ERROR_INVALID_POSITION.to_string())
    );
    assert_eq!(room.get_moves().len(), 1);
    assert_eq!(*room.get_next_turn(), StoneColor::White);
}

#[test]
fn place_stone_rejects_an_occupied_cell() {
    let mut room: GomokuRoom = two_player_room();
    let _: Result<GomokuPlaceResult, String> = GomokuDomain::place_stone(&mut room, "alice", 3, 3);
    let _: Result<GomokuPlaceResult, String> = GomokuDomain::place_stone(&mut room, "bob", 8, 8);
    let result: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "alice", 3, 3);
    assert_eq!(result.err(), Some(ERROR_POSITION_OCCUPIED.to_string()));
    assert_eq!(room.get_moves().len(), 2);
    assert_eq!(room.get_next_turn(), &StoneColor::Black);
}

#[test]
fn place_stone_records_coordinates_color_and_step_and_alternates_the_turn() {
    let mut room: GomokuRoom = two_player_room();
    let first: GomokuPlaceResult =
        GomokuDomain::place_stone(&mut room, "alice", 2, 5).expect("black move");
    assert_eq!(first.get_status(), &GameStatus::InProgress);
    assert!(!first.get_is_draw());
    assert_eq!(first.get_move_data().get_x(), 2);
    assert_eq!(first.get_move_data().get_y(), 5);
    assert_eq!(first.get_move_data().get_step(), 1);
    assert_eq!(first.get_move_data().get_color(), &StoneColor::Black);
    assert_eq!(room.get_board()[5][2], StoneColor::Black.to_value());
    assert_eq!(room.get_next_turn(), &StoneColor::White);
    let second: GomokuPlaceResult =
        GomokuDomain::place_stone(&mut room, "bob", 6, 9).expect("white");
    assert_eq!(second.get_move_data().get_step(), 2);
    assert_eq!(second.get_move_data().get_color(), &StoneColor::White);
    assert_eq!(room.get_board()[9][6], StoneColor::White.to_value());
    assert_eq!(room.get_moves().len(), 2);
    assert_eq!(room.get_moves()[1].get_step(), 2);
    assert_eq!(room.get_next_turn(), &StoneColor::Black);
    assert_eq!(room.get_status(), &GameStatus::InProgress);
}

#[test]
fn place_stone_declares_a_win_on_five_in_a_row() {
    let mut room: GomokuRoom = two_player_room();
    for index in 0usize..4 {
        let black: usize = 3 + index;
        alternate(&mut room, (black, 7), (0, index));
    }
    let winning: GomokuPlaceResult =
        GomokuDomain::place_stone(&mut room, "alice", 7, 7).expect("black completes the row");
    assert_eq!(winning.get_status(), &GameStatus::Finished);
    assert!(!winning.get_is_draw());
    assert_eq!(winning.get_winner(), StoneColor::Black);
    assert_eq!(winning.get_move_data().get_step(), 9);
    assert_eq!(room.get_status(), &GameStatus::Finished);
    assert_eq!(*room.try_get_winner(), Some(StoneColor::Black));
    let after_win: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "bob", 9, 9);
    assert_eq!(
        after_win.err(),
        Some(ERROR_GAME_NOT_IN_PROGRESS.to_string())
    );
}

#[test]
fn place_stone_declares_a_win_on_a_column_of_five() {
    let mut room: GomokuRoom = two_player_room();
    for index in 0usize..4 {
        alternate(&mut room, (7, 3 + index), (0, index));
    }
    let winning: GomokuPlaceResult =
        GomokuDomain::place_stone(&mut room, "alice", 7, 7).expect("black completes the column");
    assert_eq!(*winning.get_status(), GameStatus::Finished);
    assert_eq!(winning.get_winner(), StoneColor::Black);
    assert_eq!(*room.get_status(), GameStatus::Finished);
    assert_eq!(*room.try_get_winner(), Some(StoneColor::Black));
}

#[test]
fn place_stone_declares_a_win_on_the_main_diagonal() {
    let mut room: GomokuRoom = two_player_room();
    for index in 0usize..4 {
        alternate(&mut room, (3 + index, 3 + index), (0, index));
    }
    let winning: GomokuPlaceResult =
        GomokuDomain::place_stone(&mut room, "alice", 7, 7).expect("black completes the diagonal");
    assert_eq!(winning.get_status(), &GameStatus::Finished);
    assert_eq!(winning.get_winner(), StoneColor::Black);
    assert_eq!(*room.try_get_winner(), Some(StoneColor::Black));
}

#[test]
fn place_stone_declares_a_win_on_the_anti_diagonal() {
    let mut room: GomokuRoom = two_player_room();
    for index in 0usize..4 {
        alternate(&mut room, (7 - index, 3 + index), (0, index));
    }
    let winning: GomokuPlaceResult = GomokuDomain::place_stone(&mut room, "alice", 3, 7)
        .expect("black completes the anti diagonal");
    assert_eq!(winning.get_status(), &GameStatus::Finished);
    assert_eq!(winning.get_winner(), StoneColor::Black);
}

#[test]
fn place_stone_keeps_playing_on_four_in_a_row() {
    let mut room: GomokuRoom = two_player_room();
    for index in 0usize..3 {
        alternate(&mut room, (3 + index, 7), (0, index));
    }
    let fourth: GomokuPlaceResult =
        GomokuDomain::place_stone(&mut room, "alice", 6, 7).expect("black reaches four");
    assert_eq!(fourth.get_status(), &GameStatus::InProgress);
    assert!(!fourth.get_is_draw());
    assert_eq!(room.get_status(), &GameStatus::InProgress);
    assert!(room.try_get_winner().is_none());
    assert_eq!(room.get_next_turn(), &StoneColor::White);
    assert_eq!(fourth.get_move_data().get_step(), 7);
}

#[test]
fn place_stone_wins_across_a_gap_by_counting_both_directions() {
    let mut room: GomokuRoom = two_player_room();
    for index in 0usize..4 {
        let black: usize = 1 + index;
        if index == 2 {
            alternate(&mut room, (4, 9), (0, index));
        } else if index == 3 {
            alternate(&mut room, (5, 9), (0, index));
        } else {
            alternate(&mut room, (black, 9), (0, index));
        }
    }
    assert_eq!(room.get_moves().len(), 8);
    let bridging: GomokuPlaceResult =
        GomokuDomain::place_stone(&mut room, "alice", 3, 9).expect("black fills the gap");
    assert_eq!(*bridging.get_status(), GameStatus::Finished);
    assert_eq!(bridging.get_winner(), StoneColor::Black);
    assert_eq!(*room.get_status(), GameStatus::Finished);
}

#[test]
fn place_stone_reports_a_draw_when_the_last_empty_cell_is_filled() {
    let mut room: GomokuRoom = two_player_room();
    let mut board: Vec<Vec<u8>> = vec![];
    for y in 0usize..15 {
        let mut row: Vec<u8> = vec![];
        for x in 0usize..15 {
            let residue: usize = (x + 2 * y) % 5;
            row.push(if residue == 0 || residue == 1 { 1 } else { 2 });
        }
        board.push(row);
    }
    board[14][14] = 0;
    *room.get_mut_board() = board;
    room.set_next_turn(StoneColor::Black);
    assert_eq!(room.get_board()[14][14], 0);
    let drawing: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut room, "alice", 14, 14);
    let result: GomokuPlaceResult = drawing.expect("the last empty cell is playable");
    assert_eq!(*result.get_status(), GameStatus::Finished);
    assert!(result.get_is_draw());
    assert!(result.try_get_winner().is_none());
    assert_eq!(*room.get_status(), GameStatus::Finished);
    assert!(room.try_get_winner().is_none());
    assert!(
        room.get_board()
            .iter()
            .all(|row: &Vec<u8>| row.iter().all(|cell: &u8| *cell != 0))
    );
}

#[test]
fn a_new_room_resets_the_previous_rooms_board_and_history() {
    let mut finished: GomokuRoom = two_player_room();
    let _: Result<GomokuPlaceResult, String> =
        GomokuDomain::place_stone(&mut finished, "alice", 5, 5);
    assert_eq!(finished.get_moves().len(), 1);
    assert_eq!(finished.get_status(), &GameStatus::InProgress);
    let fresh: GomokuRoom = GomokuDomain::create_room("room-lambda", "alice");
    assert_eq!(fresh.get_room_id(), "room-lambda");
    assert!(fresh.get_moves().is_empty());
    assert_eq!(fresh.get_status(), &GameStatus::Waiting);
    assert_eq!(fresh.get_next_turn(), &StoneColor::Black);
    assert!(fresh.try_get_winner().is_none());
    assert_eq!(fresh.get_board()[5][5], 0);
    assert_eq!(fresh.get_players().len(), 1);
}

#[test]
fn stone_color_opposite_and_value_round_trip() {
    assert_eq!(StoneColor::Black.opposite(), StoneColor::White);
    assert_eq!(StoneColor::White.opposite(), StoneColor::Black);
    assert_eq!(StoneColor::Black.to_value(), 1);
    assert_eq!(StoneColor::White.to_value(), 2);
    assert_eq!(StoneColor::default(), StoneColor::Black);
    assert_eq!(
        StoneColor::Black.to_value() + StoneColor::White.to_value(),
        3
    );
}

#[test]
fn game_status_defaults_to_waiting() {
    assert_eq!(GameStatus::default(), GameStatus::Waiting);
    assert_ne!(GameStatus::Waiting, GameStatus::InProgress);
    assert_ne!(GameStatus::InProgress, GameStatus::Finished);
    assert_eq!(GameStatus::Finished, GameStatus::Finished);
}

#[test]
fn gomoku_message_type_defaults_to_unknown_and_matches_the_wire_names() {
    assert_eq!(GomokuMessageType::default(), GomokuMessageType::Unknown);
    assert_ne!(GomokuMessageType::Ping, GomokuMessageType::Pang);
    assert_ne!(GomokuMessageType::CreateRoom, GomokuMessageType::RoomState);
    let wire: String = to_string(&GomokuMessageType::PlaceStone).expect("serializes");
    assert_eq!(wire, "\"PlaceStone\"");
    let parsed: GomokuMessageType = from_str("\"Error\"").expect("deserializes");
    assert_eq!(parsed, GomokuMessageType::Error);
    assert!(from_str::<GomokuMessageType>("\"NotAType\"").is_err());
}
